use std::collections::BTreeMap;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

pub struct OutlineEntry {
    pub text: String,
    pub start_line: usize,
    pub end_line: usize,
    pub noloc: bool,
}

struct ShowNode {
    start_byte: usize,
    end_byte: usize,
    start_line: usize,
    end_line: usize,
    first_line: String,
    indented: bool,
    indent: bool,
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

    if !is_show && !is_show_if_ref && !is_hide_after && !is_show_after {
        return None;
    }

    Some(ShowNode {
        start_byte: 0,
        end_byte: 0,
        start_line: 0,
        end_line: 0,
        first_line: String::new(),
        indented: parts.contains("indented"),
        indent: parts.contains("indent"),
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

fn find_smallest_containing(show_nodes: &BTreeMap<usize, ShowNode>, start: usize, end: usize) -> Option<usize> {
    let mut best: Option<usize> = None;
    let mut best_size = usize::MAX;
    for (&key, node) in show_nodes.iter() {
        if start >= node.start_byte && end <= node.end_byte {
            let size = node.end_byte - node.start_byte;
            if size < best_size {
                best = Some(key);
                best_size = size;
            }
        }
    }
    best
}

pub fn extract_outline(source: &str, language: Language, query_src: &str) -> Vec<OutlineEntry> {
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
        let mut match_hide_range: Option<(usize, usize)> = None;
        let mut appended = false;
        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;

            if capture_name == "hide" {
                let start = node.start_byte();
                let end = node.end_byte();
                match match_hide_range {
                    None => match_hide_range = Some((start, end)),
                    Some((prev_start, prev_end)) => {
                        match_hide_range = Some((prev_start.min(start), prev_end.max(end)));
                    }
                }
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
                match_show_key = Some(start_byte);
                show_nodes.entry(start_byte).or_insert(parsed);
            }
        }

        // Apply @hide to the show node from this match (first @hide wins)
        if let (Some(key), Some((hide_start, hide_end))) = (match_show_key, match_hide_range) {
            if let Some(node) = show_nodes.get_mut(&key) {
                if node.hide_first_line.is_none() {
                    let hide_text = &source[hide_start..hide_end];
                    node.hide_first_line = Some(
                        hide_text.lines().next().unwrap_or("").trim().to_string()
                    );
                    // Only set closing line for multi-line bodies
                    if hide_text.contains('\n') {
                        node.hide_last_line = Some(
                            hide_text.lines().last().unwrap_or("").trim().to_string()
                        );
                    }
                }
            }
        }
    }

    // Apply @name: assign name to the smallest containing show node
    for (name_start, name_end, name_text) in &name_captures {
        if let Some(key) = find_smallest_containing(&show_nodes, *name_start, *name_end) {
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

    // Apply @hide: remove hidden text from first_line (before @strip so text matches)
    for node in show_nodes.values_mut() {
        if let Some(ref hide_fl) = node.hide_first_line {
            let trimmed = node.first_line.trim_end();
            if let Some(pos) = trimmed.rfind(hide_fl.as_str()) {
                node.first_line = trimmed[..pos].trim_end().to_string();
            }
        }
    }

    // Apply @strip: remove stripped text from the smallest containing show node
    for (strip_start, strip_end, strip_text) in &strip_texts {
        if let Some(key) = find_smallest_containing(&show_nodes, *strip_start, *strip_end) {
            if let Some(node) = show_nodes.get_mut(&key) {
                let with_space = format!("{} ", strip_text);
                let replaced = node.first_line.replacen(&with_space, "", 1);
                if replaced != node.first_line {
                    node.first_line = replaced;
                } else {
                    node.first_line = node.first_line.replacen(strip_text.as_str(), "", 1);
                }
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

    // Build output: for each @show, find contained @show.indented children
    let show_vec: Vec<&ShowNode> = show_nodes.values().collect();
    let mut entries = Vec::new();
    let mut skip_until: Option<usize> = None;

    for node in show_vec.iter() {
        // Skip child/toggle nodes (they're handled by their parent @show)
        if node.indented || node.indent || node.hide_after {
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

        // Find children contained within this node
        let children: Vec<&ShowNode> = show_vec.iter()
            .filter(|child| {
                (child.indented || child.indent || child.hide_after || child.show_after)
                    && child.start_byte > node.start_byte
                    && child.end_byte <= node.end_byte
            })
            .copied()
            .collect();

        let has_children = children.iter().any(|c| c.indented || c.indent);

        if !has_children {
            // Simple node, just show first line (@hide already applied)
            let text = node.first_line.trim_end().to_string();
            entries.push(OutlineEntry {
                text,
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });
        } else {
            // Block node: first line + children + closing line
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

                if !visible || !(child.indented || child.indent) {
                    continue;
                }

                let child_text = child.first_line.trim_end();
                let formatted = if child.indent {
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
