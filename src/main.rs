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
        if entry.line > 0 {
            println!("{} // L{}", entry.signature, entry.line);
        } else {
            println!("{}", entry.signature);
        }
    }
}

struct OutlineEntry {
    signature: String,
    line: usize,
}

struct MatchData {
    sig_text: String,
    line: usize,
    body_members: Vec<String>,
    body_range: Option<(usize, usize)>,
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

    let mut match_map: std::collections::BTreeMap<usize, MatchData> = std::collections::BTreeMap::new();

    while let Some(m) = matches.next() {
        let mut sig_node: Option<Node> = None;
        let mut body_node: Option<Node> = None;
        let mut name_text: Option<String> = None;

        for cap in m.captures {
            let capture_name = &query.capture_names()[cap.index as usize];
            match capture_name.as_ref() {
                "signature" => sig_node = Some(cap.node),
                "body" => body_node = Some(cap.node),
                "name" => name_text = Some(source[cap.node.byte_range()].to_string()),
                _ => {}
            }
        }

        if let Some(node) = sig_node {
            let start = node.start_byte();
            let has_body = body_node.is_some();

            // If we already have a match with body, skip plain match
            if let Some(existing) = match_map.get(&start) {
                if !existing.body_members.is_empty() && !has_body {
                    continue;
                }
            }

            let sig_text = if let Some(name) = &name_text {
                // Use @name as the display name (e.g. typedef struct { ... } Name;)
                format!("typedef {}", name)
            } else {
                let text = &source[node.byte_range()];
                let first_line = text.lines().next().unwrap_or(text);
                if let Some(pos) = first_line.find('{') {
                    first_line[..pos].trim().to_string()
                } else {
                    first_line.trim().to_string()
                }
            };

            let (body_members, body_range) = if let Some(body) = body_node {
                (collect_body_members(source, &body),
                 Some((body.start_byte(), body.end_byte())))
            } else {
                (vec![], None)
            };

            match_map.insert(start, MatchData {
                sig_text,
                line: node.start_position().row + 1,
                body_members,
                body_range,
            });
        }
    }

    // Collect body ranges to filter out nested matches
    let body_ranges: Vec<(usize, usize)> = match_map.values()
        .filter_map(|d| d.body_range)
        .collect();

    let mut entries = Vec::new();
    for (start, data) in match_map.iter() {
        // Skip if this signature is inside another entry's body
        if body_ranges.iter().any(|(bs, be)| *start > *bs && *start < *be) {
            continue;
        }
        let data = data;
        if !data.body_members.is_empty() {
            entries.push(OutlineEntry {
                signature: format!("{} {{", data.sig_text),
                line: data.line,
            });
            for member in &data.body_members {
                entries.push(OutlineEntry {
                    signature: format!("  {}", member),
                    line: 0,
                });
            }
            entries.push(OutlineEntry {
                signature: "}".to_string(),
                line: 0,
            });
        } else {
            entries.push(OutlineEntry {
                signature: data.sig_text.clone(),
                line: data.line,
            });
        }
    }

    entries
}

fn collect_body_members(source: &str, body_node: &Node) -> Vec<String> {
    let mut members = Vec::new();
    let mut cursor = body_node.walk();
    for child in body_node.named_children(&mut cursor) {
        let text = &source[child.byte_range()];
        let first_line = text.lines().next().unwrap_or(text).trim();
        let sig = if let Some(pos) = first_line.find('{') {
            first_line[..pos].trim()
        } else {
            first_line
        };
        members.push(sig.to_string());
    }
    members
}
