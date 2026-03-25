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
    hide_ranges: Vec<(usize, usize)>,
    append_range: Option<(usize, usize)>,
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
    let is_show_after = parts.contains("show_after");
    let is_hide_after = parts.contains("hide_after");

    if !is_show && !is_show_if_ref && !is_show_after && !is_hide_after {
        return None;
    }

    Some(ShowNode {
        start_byte: 0,
        end_byte: 0,
        start_line: 0,
        end_line: 0,
        hide_ranges: Vec::new(),
        append_range: None,
        noloc: parts.contains("noloc"),
        show_after: is_show_after,
        hide_after: is_hide_after,
        show_if_ref: is_show_if_ref,
        referenced: false,
        name: None,
    })
}

fn visible_text(source: &str, node: &ShowNode) -> String {
    let mut sorted_hides: Vec<_> = node.hide_ranges.clone();
    sorted_hides.sort_by_key(|(s, _)| *s);

    if sorted_hides.is_empty() {
        return source[node.start_byte..node.end_byte].to_string();
    }

    let mut result = String::new();
    let mut pos = node.start_byte;

    for (hs, he) in sorted_hides {
        if hs > pos {
            result.push_str(&source[pos..hs]);
        }
        pos = pos.max(he);
    }

    if pos < node.end_byte {
        result.push_str(&source[pos..node.end_byte]);
    }

    result
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
    let mut show_node_ids: HashMap<usize, usize> = HashMap::new();
    let mut orphan_hide_nodes: Vec<tree_sitter::Node> = Vec::new();
    let mut append_ranges: Vec<(Option<usize>, usize, usize)> = Vec::new();
    let mut ref_texts: Vec<String> = Vec::new();

    while let Some(m) = matches.next() {
        let mut match_show_ids: Vec<usize> = Vec::new();
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
                    append_ranges.push((last_show_key, node.start_byte(), node.end_byte()));
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

    // Assign @append ranges to their show nodes
    for (key, start, end) in &append_ranges {
        if let Some(key) = key {
            if let Some(node) = show_nodes.get_mut(key) {
                node.append_range = Some((*start, *end));
            }
        }
    }

    // Build output: flat list of all @show entries
    // @hide_after / @show_after toggle sibling visibility within a scope
    let mut entries = Vec::new();
    let mut hidden_until: Option<usize> = None; // end_byte of hide scope

    for node in show_nodes.values() {
        // Exit hide scope when past its boundary
        if let Some(end) = hidden_until {
            if node.start_byte >= end {
                hidden_until = None;
            }
        }

        // @show_after: clear hide scope (make subsequent siblings visible again)
        if node.show_after {
            hidden_until = None;
        }

        // @hide_after: hide subsequent siblings until scope ends
        if node.hide_after {
            // Find the containing @show to determine scope boundary
            let scope_end = show_nodes
                .values()
                .filter(|n| {
                    n.start_byte < node.start_byte
                        && n.end_byte > node.end_byte
                        && !n.hide_after
                        && !n.show_after
                })
                .min_by_key(|n| n.end_byte - n.start_byte)
                .map(|n| n.end_byte);
            hidden_until = scope_end;
        }

        // Skip toggle-only nodes and hidden nodes
        if node.hide_after && !node.show_if_ref {
            continue;
        }
        if node.show_after && !node.show_if_ref {
            continue;
        }
        if node.show_if_ref && !node.referenced {
            continue;
        }
        if hidden_until.is_some() {
            continue;
        }

        let mut text = visible_text(source, node);
        if let Some((s, e)) = node.append_range {
            text.push_str(source[s..e].trim());
        }
        let text = text
            .lines()
            .map(|l| l.trim_end())
            .filter(|l| !l.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        entries.push(OutlineEntry {
            text,
            start_line: node.start_line,
            end_line: node.end_line,
            noloc: node.noloc,
        });
    }

    entries
}
