use std::collections::{BTreeMap, HashMap};
use std::io::{self, Write};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

pub struct OutlineEntry {
    pub ranges: Vec<(usize, usize)>,
    pub start_line: usize,
    pub end_line: usize,
    pub noloc: bool,
}

impl OutlineEntry {
    pub fn write_to(&self, source: &str, w: &mut impl Write) -> io::Result<()> {
        let mut first = true;
        for &(s, e) in &self.ranges {
            for line in source[s..e].lines() {
                let trimmed = line.trim_end();
                if trimmed.is_empty() {
                    continue;
                }
                if !first {
                    writeln!(w)?;
                }
                write!(w, "{}", trimmed)?;
                first = false;
            }
        }
        Ok(())
    }
}

struct ShowNode {
    start_byte: usize,
    end_byte: usize,
    start_line: usize,
    end_line: usize,
    hide_ranges: Vec<(usize, usize)>,
    append_range: Option<(usize, usize)>,
    show: bool,      // emit as entry
    noloc: bool,
    show_after: bool, // toggle: make subsequent siblings visible
    hide_after: bool, // toggle: make subsequent siblings hidden
    show_if_ref: bool,
    referenced: bool,
    name: Option<String>,
}

fn parse_capture(name: &str, node: &tree_sitter::Node) -> Option<ShowNode> {
    let parts: std::collections::HashSet<&str> = name.split('.').collect();

    let is_show = parts.contains("show");
    let is_show_if_ref = parts.contains("show_if_ref");
    let is_show_after = parts.contains("show_after");
    let is_hide_after = parts.contains("hide_after");

    if !is_show && !is_show_if_ref && !is_show_after && !is_hide_after {
        return None;
    }

    Some(ShowNode {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
        hide_ranges: Vec::new(),
        append_range: None,
        show: is_show || is_show_if_ref,
        noloc: parts.contains("noloc"),
        show_after: is_show_after,
        hide_after: is_hide_after,
        show_if_ref: is_show_if_ref,
        referenced: false,
        name: None,
    })
}

fn visible_ranges(node: &ShowNode) -> Vec<(usize, usize)> {
    let mut sorted_hides: Vec<_> = node.hide_ranges.clone();
    sorted_hides.sort_by_key(|(s, _)| *s);

    if sorted_hides.is_empty() {
        return vec![(node.start_byte, node.end_byte)];
    }

    let mut ranges = Vec::new();
    let mut pos = node.start_byte;

    for (hs, he) in sorted_hides {
        if hs > pos {
            ranges.push((pos, hs));
        }
        pos = pos.max(he);
    }

    if pos < node.end_byte {
        ranges.push((pos, node.end_byte));
    }

    ranges
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

            if let Some(parsed) = parse_capture(capture_name, &node) {
                last_show_key = Some(parsed.start_byte);
                match_show_ids.push(node.id());
                show_node_ids.insert(node.id(), parsed.start_byte);
                show_nodes.entry(parsed.start_byte).or_insert(parsed);
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

    // Walk the AST tree to build output, using tree structure for sibling scoping
    let mut entries = Vec::new();
    collect_entries(
        tree.root_node(),
        &show_node_ids,
        &show_nodes,
        &mut entries,
    );

    entries
}

fn collect_entries(
    parent: tree_sitter::Node,
    show_node_ids: &HashMap<usize, usize>,
    show_nodes: &BTreeMap<usize, ShowNode>,
    entries: &mut Vec<OutlineEntry>,
) {
    let mut hidden = false;

    for i in 0..parent.child_count() as u32 {
        let child = parent.child(i).unwrap();

        if let Some(&start_byte) = show_node_ids.get(&child.id()) {
            if let Some(node) = show_nodes.get(&start_byte) {
                if node.show_after {
                    hidden = false;
                }
                if node.hide_after {
                    hidden = true;
                }

                let should_emit = node.show
                    && !(node.show_if_ref && !node.referenced)
                    && !hidden;

                if should_emit {
                    let mut ranges = visible_ranges(node);
                    if let Some(append) = node.append_range {
                        ranges.push(append);
                    }
                    entries.push(OutlineEntry {
                        ranges,
                        start_line: node.start_line,
                        end_line: node.end_line,
                        noloc: node.noloc,
                    });
                }
            }
        }

        // Recurse to find deeper captures
        collect_entries(child, show_node_ids, show_nodes, entries);
    }
}
