use std::collections::BTreeMap;
use std::io::Read;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

fn main() {
    let lang_arg = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: zat-tree-sitter-viewer <lang>");
        std::process::exit(1);
    });

    let (language, query_src) = match lang_arg.as_str() {
        "go" => (tree_sitter_go::LANGUAGE.into(), include_str!("../queries/go.scm")),
        "c" => (tree_sitter_c::LANGUAGE.into(), include_str!("../queries/c.scm")),
        "cpp" | "cc" | "cxx" => (tree_sitter_cpp::LANGUAGE.into(), include_str!("../queries/cpp.scm")),
        "js" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), include_str!("../queries/javascript.scm")),
        "ts" | "tsx" => (tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), include_str!("../queries/typescript.scm")),
        "python" | "py" => (tree_sitter_python::LANGUAGE.into(), include_str!("../queries/python.scm")),
        "rust" | "rs" => (tree_sitter_rust::LANGUAGE.into(), include_str!("../queries/rust.scm")),
        "java" => (tree_sitter_java::LANGUAGE.into(), include_str!("../queries/java.scm")),
        other => {
            eprintln!("Unsupported language: {}", other);
            std::process::exit(1);
        }
    };

    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source).expect("Failed to read stdin");

    let entries = extract_outline(&source, language, query_src);
    for entry in &entries {
        print!("{}", entry.text);
        if entry.start_line > 0 {
            if entry.end_line > entry.start_line {
                println!(" // L{}-L{}", entry.start_line, entry.end_line);
            } else {
                println!(" // L{}", entry.start_line);
            }
        } else {
            println!();
        }
    }
}

struct OutlineEntry {
    text: String,
    start_line: usize,
    end_line: usize,
}

struct ShowNode {
    start_byte: usize,
    end_byte: usize,
    start_line: usize,
    end_line: usize,
    first_line: String,
    last_line: String,
    indent: bool,
}

fn first_line_of(source: &str, node: &Node) -> String {
    let text = &source[node.byte_range()];
    let line = text.lines().next().unwrap_or(text).trim();
    // Trim trailing opening brace for cleaner display, we'll add it back for blocks
    line.to_string()
}

fn last_line_of(source: &str, node: &Node) -> String {
    let text = &source[node.byte_range()];
    text.lines().last().unwrap_or("").trim().to_string()
}

fn extract_outline(source: &str, language: Language, query_src: &str) -> Vec<OutlineEntry> {
    let mut parser = Parser::new();
    parser.set_language(&language).expect("Failed to set language");

    let tree = match parser.parse(source, None) {
        Some(t) => t,
        None => return vec![],
    };

    let query = match Query::new(&language, query_src) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Query error: {}", e);
            return vec![];
        }
    };

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());

    // Collect all @show and @show.indent nodes
    let mut show_nodes: BTreeMap<usize, ShowNode> = BTreeMap::new();

    while let Some(m) = matches.next() {
        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;
            let indent = capture_name == "show.indent";

            if capture_name == "show" || capture_name == "show.indent" {
                let start_byte = node.start_byte();
                // Don't overwrite if already captured (first match wins)
                show_nodes.entry(start_byte).or_insert(ShowNode {
                    start_byte,
                    end_byte: node.end_byte(),
                    start_line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                    first_line: first_line_of(source, &node),
                    last_line: last_line_of(source, &node),
                    indent,
                });
            }
        }
    }

    // Build output: for each @show, find contained @show.indent children
    let show_vec: Vec<&ShowNode> = show_nodes.values().collect();
    let mut entries = Vec::new();
    let mut skip_until: Option<usize> = None;

    for node in show_vec.iter() {
        // Skip @show.indent nodes (they're handled by their parent @show)
        if node.indent {
            continue;
        }

        // Skip nodes nested inside another @show's range (already processed)
        if let Some(end) = skip_until {
            if node.start_byte < end {
                continue;
            }
            skip_until = None;
        }

        // Find @show.indent children contained within this node
        let children: Vec<&ShowNode> = show_vec.iter()
            .filter(|child| {
                child.indent
                    && child.start_byte > node.start_byte
                    && child.end_byte <= node.end_byte
            })
            .copied()
            .collect();

        if children.is_empty() {
            // Simple node, just show first line (trim trailing '{' since we don't expand)
            let text = node.first_line.trim_end_matches('{').trim_end().to_string();
            entries.push(OutlineEntry {
                text,
                start_line: node.start_line,
                end_line: node.end_line,
            });
        } else {
            // Block node: first line + indented children + closing line
            entries.push(OutlineEntry {
                text: node.first_line.clone(),
                start_line: node.start_line,
                end_line: node.end_line,
            });

            for child in &children {
                // Trim trailing '{' for indented items that aren't expanded
                let child_text = child.first_line.trim_end_matches('{').trim_end();
                entries.push(OutlineEntry {
                    text: format!("  {}", child_text),
                    start_line: child.start_line,
                    end_line: child.end_line,
                });
            }

            // Show closing line if it looks like a delimiter (e.g., "}", "};")
            let last = node.last_line.trim_end_matches(';').trim();
            if last == "}" || last == ")" {
                entries.push(OutlineEntry {
                    text: node.last_line.clone(),
                    start_line: 0,
                    end_line: 0,
                });
            }

            skip_until = Some(node.end_byte);
        }
    }

    entries
}
