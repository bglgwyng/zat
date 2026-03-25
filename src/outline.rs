use std::collections::{BTreeMap, HashMap};
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
    hide_ranges: Vec<(usize, usize)>,
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

struct Outline<'a> {
    source: &'a str,
    show_nodes: &'a BTreeMap<usize, ShowNode>,
}

impl Outline<'_> {
    fn visible_text(&self, node: &ShowNode) -> String {
        let mut sorted_hides: Vec<_> = node.hide_ranges.clone();
        sorted_hides.sort_by_key(|(s, _)| *s);

        if sorted_hides.is_empty() {
            return self.source[node.start_byte..node.end_byte].to_string();
        }

        let bytes = self.source.as_bytes();
        let mut result = String::new();
        let mut pos = node.start_byte;

        for (hs, he) in sorted_hides {
            if hs > pos {
                result.push_str(&self.source[pos..hs]);
            }

            // Find preservable @show nodes within this hide and recurse
            for (_, child) in self.show_nodes.range(hs..he) {
                if child.start_byte < hs || child.end_byte > he {
                    continue;
                }
                if child.start_byte == node.start_byte && child.end_byte == node.end_byte {
                    continue;
                }
                if child.indented
                    || child.indent
                    || child.show_if_ref
                    || child.show_after
                    || child.hide_after
                {
                    continue;
                }

                if child.start_byte > 0 && bytes[child.start_byte - 1] == b'\n' {
                    result.push('\n');
                }
                result.push_str(&self.visible_text(child));
                if child.end_byte < bytes.len() && bytes[child.end_byte] == b'\n' {
                    result.push('\n');
                }
            }

            pos = he;
        }

        if pos < node.end_byte {
            result.push_str(&self.source[pos..node.end_byte]);
        }

        result
    }
}

pub fn extract_outline(source: &str, language: Language, query_src: &str) -> Vec<OutlineEntry> {
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("Failed to set language");

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

    let mut show_nodes: BTreeMap<usize, ShowNode> = BTreeMap::new();
    let mut show_node_ids: HashMap<usize, usize> = HashMap::new(); // node.id() -> start_byte
    let mut orphan_hide_nodes: Vec<tree_sitter::Node> = Vec::new();
    let mut append_texts: Vec<(Option<usize>, String)> = Vec::new();
    let mut ref_texts: Vec<String> = Vec::new();

    while let Some(m) = matches.next() {
        let mut match_show_ids: Vec<usize> = Vec::new(); // node IDs of shows in this match
        let mut match_hide_nodes: Vec<tree_sitter::Node> = Vec::new();
        let mut match_name: Option<String> = None;
        let mut last_show_key: Option<usize> = None;
        let mut appended = false;

        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;

            if capture_name == "hide" {
                match_hide_nodes.push(node);
                continue;
            }

            if capture_name == "append" {
                if !appended {
                    let text = source[node.byte_range()].trim().to_string();
                    append_texts.push((last_show_key, text));
                    appended = true;
                }
                continue;
            }

            if capture_name == "name" {
                match_name = Some(source[node.byte_range()].trim().to_string());
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
                last_show_key = Some(start_byte);
                match_show_ids.push(node.id());
                show_node_ids.insert(node.id(), start_byte);
                show_nodes.entry(start_byte).or_insert(parsed);
            }
        }

        // Assign @hide to nearest @show ancestor within this match
        for hide_node in match_hide_nodes {
            let (hs, he) = (hide_node.start_byte(), hide_node.end_byte());
            let mut assigned = false;
            let mut ancestor = hide_node.parent();
            while let Some(a) = ancestor {
                if match_show_ids.contains(&a.id()) {
                    let start_byte = show_node_ids[&a.id()];
                    show_nodes
                        .get_mut(&start_byte)
                        .unwrap()
                        .hide_ranges
                        .push((hs, he));
                    assigned = true;
                    break;
                }
                ancestor = a.parent();
            }
            if !assigned {
                orphan_hide_nodes.push(hide_node);
            }
        }

        // Assign @name to the (unique) @show_if_ref from this match
        if let Some(name_text) = match_name {
            if let Some(key) = last_show_key {
                if let Some(node) = show_nodes.get_mut(&key) {
                    node.name = Some(name_text);
                }
            }
        }
    }

    // Assign orphan @hide to nearest @show ancestor via parent walk
    for hide_node in &orphan_hide_nodes {
        let (hs, he) = (hide_node.start_byte(), hide_node.end_byte());
        let mut ancestor = hide_node.parent();
        while let Some(a) = ancestor {
            if let Some(&start_byte) = show_node_ids.get(&a.id()) {
                show_nodes
                    .get_mut(&start_byte)
                    .unwrap()
                    .hide_ranges
                    .push((hs, he));
                break;
            }
            ancestor = a.parent();
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

    // Build visible text for each node by recursively processing hides
    let outline = Outline {
        source,
        show_nodes: &show_nodes,
    };
    let updates: Vec<(usize, String)> = show_nodes
        .iter()
        .filter(|(_, node)| !node.hide_ranges.is_empty())
        .map(|(&key, node)| {
            let text = outline
                .visible_text(node)
                .lines()
                .map(|l| l.trim_end())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            (key, text)
        })
        .collect();
    for (key, text) in updates {
        show_nodes.get_mut(&key).unwrap().text = text;
    }

    // Apply @append: append text to the show node from the same match
    for (key, append_text) in &append_texts {
        if let Some(key) = key {
            if let Some(node) = show_nodes.get_mut(key) {
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
        if node.indented || node.indent || node.hide_after {
            continue;
        }

        if node.show_if_ref && !node.referenced {
            skip_until = Some(node.end_byte);
            continue;
        }

        if let Some(end) = skip_until {
            if node.start_byte < end {
                continue;
            }
            skip_until = None;
        }

        let children: Vec<&ShowNode> = show_vec
            .iter()
            .filter(|child| {
                (child.indented || child.indent || child.hide_after || child.show_after)
                    && child.start_byte > node.start_byte
                    && child.end_byte <= node.end_byte
            })
            .copied()
            .collect();

        let has_children = children.iter().any(|c| c.indented || c.indent);

        if !has_children {
            entries.push(OutlineEntry {
                text: node.text.trim().to_string(),
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });
        } else {
            let header = node.text.trim_end();
            let (first_line, closing) = match header.find('\n') {
                Some(pos) => (
                    &header[..pos],
                    Some(header[pos + 1..].trim_start_matches('\n')),
                ),
                None => (&header[..], None),
            };
            entries.push(OutlineEntry {
                text: first_line.to_string(),
                start_line: node.start_line,
                end_line: node.end_line,
                noloc: node.noloc,
            });

            let mut visible = true;
            for child in &children {
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

            if let Some(closing) = closing {
                if !closing.is_empty() {
                    entries.push(OutlineEntry {
                        text: closing.to_string(),
                        start_line: node.end_line,
                        end_line: node.end_line,
                        noloc: true,
                    });
                }
            }

            skip_until = Some(node.end_byte);
        }
    }

    entries
}
