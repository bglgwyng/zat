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
        if entry.start_line > 0 && !entry.noloc {
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
    noloc: bool,
}

struct ShowNode {
    start_byte: usize,
    end_byte: usize,
    start_line: usize,
    end_line: usize,
    first_line: String,
    last_line: String,
    indent: bool,
    noindent: bool,
    noloc: bool,
    show_after: bool,
    hide_after: bool,
    show_if_ref: bool,
    referenced: bool,
    name: Option<String>,
    hide_first_line: Option<String>,
    hide_last_line: Option<String>,
}

fn parse_capture(name: &str) -> Option<ShowNode> {
    let parts: std::collections::HashSet<&str> = name.split('.').collect();

    // Must have "show", "show_if_ref", or "hide_after" as base
    let is_show = parts.contains("show");
    let is_show_if_ref = parts.contains("show_if_ref");
    let is_hide_after = parts.contains("hide_after");
    let is_show_after = parts.contains("show_after");

    if !is_show && !is_show_if_ref && !is_hide_after {
        return None;
    }

    Some(ShowNode {
        start_byte: 0,
        end_byte: 0,
        start_line: 0,
        end_line: 0,
        first_line: String::new(),
        last_line: String::new(),
        indent: parts.contains("indent"),
        noindent: parts.contains("noindent"),
        noloc: parts.contains("noloc"),
        show_after: is_show_after,
        hide_after: is_hide_after,
        show_if_ref: is_show_if_ref,
        referenced: false,
        name: None,
        hide_first_line: None,
        hide_last_line: None,
    })
}

fn first_line_of(source: &str, node: &Node) -> String {
    let text = &source[node.byte_range()];
    let line = text.lines().next().unwrap_or(text).trim();
    line.to_string()
}

fn last_line_of(source: &str, node: &Node) -> String {
    let text = &source[node.byte_range()];
    text.lines().last().unwrap_or("").trim().to_string()
}

fn trim_body_with<'a>(line: &'a str, body_first_line: Option<&str>) -> &'a str {
    match body_first_line {
        Some(body_fl) => {
            let trimmed = line.trim_end();
            if let Some(pos) = trimmed.rfind(body_fl) {
                trimmed[..pos].trim_end()
            } else {
                trimmed
            }
        }
        None => line.trim_end(),
    }
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

    // Collect all @show, @show_if_ref, and @hide_after nodes
    let mut show_nodes: BTreeMap<usize, ShowNode> = BTreeMap::new();
    let mut strip_texts: Vec<(usize, usize, String)> = Vec::new();
    let mut append_texts: Vec<(Option<usize>, String)> = Vec::new();
    let mut name_captures: Vec<(usize, usize, String)> = Vec::new();
    let mut ref_texts: Vec<String> = Vec::new();

    while let Some(m) = matches.next() {
        let mut match_show_key: Option<usize> = None;
        let mut match_body: Option<(String, String)> = None;
        let mut appended = false;
        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;

            if capture_name == "hide" {
                match_body = Some((first_line_of(source, &node), last_line_of(source, &node)));
                continue;
            }

            if capture_name == "strip" {
                let text = source[node.byte_range()].trim().to_string();
                strip_texts.push((node.start_byte(), node.end_byte(), text));
                continue;
            }

            if capture_name == "append" {
                if !appended {
                    let text = source[node.byte_range()].trim().to_string();
                    append_texts.push((match_show_key, text));
                    appended = true;
                }
                continue;
            }

            if capture_name == "name" {
                let text = source[node.byte_range()].trim().to_string();
                name_captures.push((node.start_byte(), node.end_byte(), text));
                continue;
            }

            if capture_name == "ref" {
                let text = source[node.byte_range()].trim().to_string();
                ref_texts.push(text);
                continue;
            }

            if let Some(mut parsed) = parse_capture(capture_name) {
                let start_byte = node.start_byte();
                parsed.start_byte = start_byte;
                parsed.end_byte = node.end_byte();
                parsed.start_line = node.start_position().row + 1;
                parsed.end_line = node.end_position().row + 1;
                parsed.first_line = first_line_of(source, &node);
                parsed.last_line = last_line_of(source, &node);
                match_show_key = Some(start_byte);
                show_nodes.entry(start_byte).or_insert(parsed);
            }
        }

        // Apply @hide to the show node from this match
        if let (Some(key), Some((hide_fl, hide_ll))) = (match_show_key, match_body) {
            if let Some(node) = show_nodes.get_mut(&key) {
                node.hide_first_line = Some(hide_fl);
                node.hide_last_line = Some(hide_ll);
            }
        }
    }

    // Apply @name: assign name to the smallest containing show node
    for (name_start, name_end, name_text) in &name_captures {
        let mut best: Option<usize> = None;
        let mut best_size = usize::MAX;
        for (&key, node) in show_nodes.iter() {
            if *name_start >= node.start_byte && *name_end <= node.end_byte {
                let size = node.end_byte - node.start_byte;
                if size < best_size {
                    best = Some(key);
                    best_size = size;
                }
            }
        }
        if let Some(key) = best {
            if let Some(node) = show_nodes.get_mut(&key) {
                node.name = Some(name_text.clone());
            }
        }
    }

    // Apply @ref: mark matching nodes as referenced
    for ref_text in &ref_texts {
        for node in show_nodes.values_mut() {
            if node.name.as_deref() == Some(ref_text.as_str()) {
                node.referenced = true;
                break;
            }
        }
    }

    // Apply @strip: remove stripped text from the smallest containing show node
    for (strip_start, strip_end, strip_text) in &strip_texts {
        let mut best: Option<usize> = None;
        let mut best_size = usize::MAX;
        for (&key, node) in show_nodes.iter() {
            if *strip_start >= node.start_byte && *strip_end <= node.end_byte {
                let size = node.end_byte - node.start_byte;
                if size < best_size {
                    best = Some(key);
                    best_size = size;
                }
            }
        }
        if let Some(key) = best {
            if let Some(node) = show_nodes.get_mut(&key) {
                node.first_line = node.first_line
                    .replace(&format!("{} ", strip_text), "")
                    .replace(strip_text.as_str(), "");
            }
        }
    }

    // Apply @append: append text to the show node from the same match
    for (key, append_text) in &append_texts {
        if let Some(key) = key {
            if let Some(node) = show_nodes.get_mut(key) {
                node.first_line.push_str(append_text);
            }
        }
    }

    // Build output: for each @show, find contained @show.indent children
    let show_vec: Vec<&ShowNode> = show_nodes.values().collect();
    let mut entries = Vec::new();
    let mut skip_until: Option<usize> = None;

    for node in show_vec.iter() {
        // Skip indent/hide_after nodes (they're handled by their parent @show)
        if node.indent || node.hide_after {
            continue;
        }

        // Skip show_if_ref nodes that were not referenced
        if node.show_if_ref && !node.referenced {
            continue;
        }

        // Skip nodes nested inside another @show's range (already processed)
        if let Some(end) = skip_until {
            if node.start_byte < end {
                continue;
            }
            skip_until = None;
        }

        // Find children contained within this node (both @show.indent and visibility toggles)
        let children: Vec<&ShowNode> = show_vec.iter()
            .filter(|child| {
                (child.indent || child.hide_after || child.show_after)
                    && child.start_byte > node.start_byte
                    && child.end_byte <= node.end_byte
            })
            .copied()
            .collect();

        // Filter out children that are only hide_after (no show) with no indent children
        let has_indent_children = children.iter().any(|c| c.indent);

        if !has_indent_children {
            // Simple node, just show first line (hide body if @hide present)
            let text = trim_body_with(&node.first_line, node.hide_first_line.as_deref()).to_string();
            entries.push(OutlineEntry {
                text,
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });
        } else {
            // Block node: first line + indented children + closing line
            entries.push(OutlineEntry {
                text: node.first_line.clone(),
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });

            let mut visible = true;
            for child in &children {
                // Handle visibility toggles
                if child.hide_after {
                    visible = false;
                    continue;
                }
                if child.show_after {
                    visible = true;
                }

                if !visible || !child.indent {
                    continue;
                }

                let child_text = trim_body_with(&child.first_line, child.hide_first_line.as_deref());
                let formatted = if child.noindent {
                    child_text.to_string()
                } else {
                    format!("  {}", child_text)
                };
                entries.push(OutlineEntry {
                    text: formatted,
                    start_line: child.start_line,
                    end_line: child.end_line,
                    noloc: child.noloc,
                });
            }

            // Show closing line from @hide
            if let Some(ref hide_ll) = node.hide_last_line {
                entries.push(OutlineEntry {
                    text: hide_ll.clone(),
                    start_line: 0,
                    end_line: 0,
                    noloc: true,
                });
            }

            skip_until = Some(node.end_byte);
        }
    }

    entries
}
