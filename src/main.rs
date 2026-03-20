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
        if entry.start_line > 0 {
            if entry.end_line > entry.start_line {
                println!("{} // L{}-L{}", entry.signature, entry.start_line, entry.end_line);
            } else {
                println!("{} // L{}", entry.signature, entry.start_line);
            }
        } else {
            println!("{}", entry.signature);
        }
    }
}

struct OutlineEntry {
    signature: String,
    start_line: usize,
    end_line: usize,
}

struct MatchData {
    sig_text: String,
    start_line: usize,
    end_line: usize,
    members: Vec<Member>,
    body_range: Option<(usize, usize)>,
}

struct Member {
    text: String,
    no_indent: bool,
    start_line: usize,
    end_line: usize,
}

fn format_member(source: &str, node: &Node) -> Member {
    let text = &source[node.byte_range()];
    let first_line = text.lines().next().unwrap_or(text).trim();

    let start_line = node.start_position().row + 1;
    let end_line = node.end_position().row + 1;

    if node.kind() == "access_specifier" {
        return Member {
            text: format!("{}:", first_line),
            no_indent: true,
            start_line: 0,
            end_line: 0,
        };
    }

    let sig = if let Some(pos) = first_line.find('{') {
        first_line[..pos].trim()
    } else {
        first_line
    };
    Member {
        text: sig.to_string(),
        no_indent: false,
        start_line,
        end_line,
    }
}

fn collect_body_members(source: &str, body_node: &Node) -> Vec<Member> {
    let mut members = Vec::new();
    let mut cursor = body_node.walk();
    for child in body_node.named_children(&mut cursor) {
        members.push(format_member(source, &child));
    }
    members
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

            if let Some(existing) = match_map.get(&start) {
                if !existing.members.is_empty() && !has_body {
                    continue;
                }
            }

            let sig_text = if let Some(name) = &name_text {
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

            let (members, body_range) = if let Some(body) = body_node {
                (collect_body_members(source, &body),
                 Some((body.start_byte(), body.end_byte())))
            } else {
                (vec![], None)
            };

            match_map.insert(start, MatchData {
                sig_text,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
                members,
                body_range,
            });
        }
    }

    let body_ranges: Vec<(usize, usize)> = match_map.values()
        .filter_map(|d| d.body_range)
        .collect();

    let mut entries = Vec::new();
    for (start, data) in match_map.iter() {
        if body_ranges.iter().any(|(bs, be)| *start > *bs && *start < *be) {
            continue;
        }
        if !data.members.is_empty() {
            let uses_braces = !data.sig_text.ends_with(':');
            let header = if uses_braces {
                format!("{} {{", data.sig_text)
            } else {
                data.sig_text.clone()
            };
            entries.push(OutlineEntry {
                signature: header,
                start_line: data.start_line,
                end_line: data.end_line,
            });
            for member in &data.members {
                let sig = if member.no_indent {
                    member.text.clone()
                } else {
                    format!("  {}", member.text)
                };
                entries.push(OutlineEntry {
                    signature: sig,
                    start_line: member.start_line,
                    end_line: member.end_line,
                });
            }
            if uses_braces {
                entries.push(OutlineEntry {
                    signature: "}".to_string(),
                    start_line: 0,
                    end_line: 0,
                });
            }
        } else {
            entries.push(OutlineEntry {
                signature: data.sig_text.clone(),
                start_line: data.start_line,
                end_line: data.end_line,
            });
        }
    }

    entries
}
