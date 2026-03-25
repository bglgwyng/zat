use std::collections::BTreeMap;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

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
    text: String,
    hide_ranges: Vec<(usize, usize)>, // absolute byte ranges to remove
    indented: bool,
    indent: bool,
    noloc: bool,
    show_after: bool,
    hide_after: bool,
    show_if_ref: bool,
    referenced: bool,
    name: Option<String>,
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
        text: String::new(),
        hide_ranges: Vec::new(),
        indented: parts.contains("indented"),
        indent: parts.contains("indent"),
        noloc: parts.contains("noloc"),
        show_after: is_show_after,
        hide_after: is_hide_after,
        show_if_ref: is_show_if_ref,
        referenced: false,
        name: None,
    })
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
    let mut strip_ranges: Vec<(usize, usize)> = Vec::new();
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
                strip_ranges.push((node.start_byte(), node.end_byte()));
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
                parsed.text = source[node.byte_range()].to_string();
                match_show_key = Some(start_byte);
                show_nodes.entry(start_byte).or_insert(parsed);
            }
        }

        // Store @hide range on the show node from this match
        if let (Some(key), Some(hide_range)) = (match_show_key, match_hide_range) {
            if let Some(node) = show_nodes.get_mut(&key) {
                node.hide_ranges.push(hide_range);
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

    // Apply @hide and @strip: remove ranges from text
    // Collect all removal ranges (hide + strip) per show node, sort, and remove
    for node in show_nodes.values_mut() {
        let mut ranges: Vec<(usize, usize)> = node.hide_ranges.clone();

        // Add strip ranges that fall within this node
        for &(s, e) in &strip_ranges {
            if s >= node.start_byte && e <= node.end_byte {
                // Also consume trailing space after strip
                let strip_end = if e < node.end_byte && source.as_bytes()[e] == b' ' {
                    e + 1
                } else {
                    e
                };
                ranges.push((s, strip_end));
            }
        }

        if ranges.is_empty() {
            continue;
        }

        // Sort by start position
        ranges.sort_by_key(|&(s, _)| s);

        // Rebuild text by skipping ranges
        let mut result = String::new();
        let mut pos = node.start_byte;
        for (start, end) in &ranges {
            if *start > pos {
                result.push_str(&source[pos..*start]);
            }
            pos = pos.max(*end);
        }
        if pos < node.end_byte {
            result.push_str(&source[pos..node.end_byte]);
        }
        // Clean up blank lines left by range removal
        let has_hide = !node.hide_ranges.is_empty();
        node.text = result
            .lines()
            .map(|l| {
                if has_hide {
                    l.trim_end_matches(';').trim_end()
                } else {
                    l.trim_end()
                }
            })
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Apply @append: append text to the show node from the same match
    for (key, append_text) in &append_texts {
        if let Some(key) = key {
            if let Some(node) = show_nodes.get_mut(key) {
                // Append to first line
                if let Some(newline_pos) = node.text.find('\n') {
                    node.text.insert_str(newline_pos, append_text);
                } else {
                    node.text.push_str(append_text);
                }
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
            // Simple node: show full text
            entries.push(OutlineEntry {
                text: node.text.trim().to_string(),
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });
        } else {
            // Block node: first line + children
            let header = node.text.trim_end();
            entries.push(OutlineEntry {
                text: header.to_string(),
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

                let child_text = child.text.trim();
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

            skip_until = Some(node.end_byte);
        }
    }

    entries
}
