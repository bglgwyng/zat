use std::collections::{BTreeMap, HashMap};
use std::io::{self, Write};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

pub struct VisibleRange {
    pub start_byte: usize,
    pub end_byte: usize,
    pub noloc: bool,
}

pub fn write_outline(source: &str, ranges: &[VisibleRange], prefix: &str, w: &mut impl Write) -> io::Result<()> {
    // Pre-compute line starts for byte→line lookup
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(source.bytes().enumerate().filter_map(|(i, b)| if b == b'\n' { Some(i + 1) } else { None }))
        .collect();
    let byte_to_line = |byte: usize| -> usize {
        line_starts.partition_point(|&start| start <= byte)
    };

    let mut prev_line: Option<usize> = None;
    // Track line number range for the current output line
    let mut line_start_num: usize = 0;
    let mut line_end_num: usize = 0;
    let mut line_has_loc = false;

    for range in ranges {
        let text = source[range.start_byte..range.end_byte].trim_end();
        if text.is_empty() {
            continue;
        }

        let start_line = byte_to_line(range.start_byte);
        let new_line = prev_line.map_or(true, |prev| start_line > prev);

        if new_line {
            // Emit line number annotation for the previous output line
            if prev_line.is_some() {
                if line_has_loc {
                    if line_end_num > line_start_num {
                        write!(w, " // L{}-L{}", line_start_num, line_end_num)?;
                    } else {
                        write!(w, " // L{}", line_start_num)?;
                    }
                }
                writeln!(w)?;
            }
            // Compute source line indent
            let line_start = if start_line > 1 { line_starts[start_line - 1] } else { 0 };
            let line_text = &source[line_start..];
            let indent_len = line_text.len() - line_text.trim_start().len();
            write!(w, "{}{}", prefix, &source[line_start..line_start + indent_len])?;
            write!(w, "{}", text.trim_start())?;
            line_start_num = start_line;
            line_end_num = start_line;
            line_has_loc = !range.noloc;
        } else {
            write!(w, "{}", text)?;
        }

        if !range.noloc {
            line_has_loc = true;
        }
        let last_byte = range.start_byte + text.len();
        let end_line = byte_to_line(last_byte.saturating_sub(1));
        if end_line > line_end_num {
            line_end_num = end_line;
        }
        prev_line = Some(end_line);
    }

    // Emit annotation for the last output line
    if prev_line.is_some() {
        if line_has_loc {
            if line_end_num > line_start_num {
                write!(w, " // L{}-L{}", line_start_num, line_end_num)?;
            } else {
                write!(w, " // L{}", line_start_num)?;
            }
        }
        writeln!(w)?;
    }

    Ok(())
}

struct ShowNode {
    start_byte: usize,
    end_byte: usize,
    hide_ranges: Vec<(usize, usize)>,
    show: bool,
    noloc: bool,
    show_after: bool,
    hide_after: bool,
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
        hide_ranges: Vec::new(),
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

pub fn extract_outline(source: &str, language: Language, query_src: &str) -> Vec<VisibleRange> {
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
    let mut ref_texts: Vec<String> = Vec::new();

    while let Some(m) = matches.next() {
        let mut match_show_ids: Vec<usize> = Vec::new();
        let mut match_hide_nodes: Vec<tree_sitter::Node> = Vec::new();
        let mut match_name: Option<String> = None;
        let mut last_show_key: Option<usize> = None;

        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;

            if capture_name == "hide" {
                match_hide_nodes.push(node);
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

    // Walk the AST tree to collect visible ranges
    let mut ranges = Vec::new();
    collect_ranges(
        tree.root_node(),
        &show_node_ids,
        &show_nodes,
        &mut ranges,
    );

    ranges
}

fn collect_ranges(
    parent: tree_sitter::Node,
    show_node_ids: &HashMap<usize, usize>,
    show_nodes: &BTreeMap<usize, ShowNode>,
    ranges: &mut Vec<VisibleRange>,
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
                    for (s, e) in visible_ranges(node) {
                        ranges.push(VisibleRange {
                            start_byte: s,
                            end_byte: e,
                            noloc: node.noloc,
                        });
                    }
                }
            }
        }

        // Recurse to find deeper captures
        collect_ranges(child, show_node_ids, show_nodes, ranges);
    }
}
