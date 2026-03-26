use std::collections::HashMap;
use std::io::{self, Write};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

pub struct VisibleRange {
    pub start_byte: usize,
    pub end_byte: usize,
    pub noloc: bool,
}

pub fn write_outline(
    source: &str,
    ranges: &[VisibleRange],
    prefix: &str,
    w: &mut impl Write,
) -> io::Result<()> {
    let line_starts: Vec<usize> = std::iter::once(0)
        .chain(
            source
                .bytes()
                .enumerate()
                .filter_map(|(i, b)| if b == b'\n' { Some(i + 1) } else { None }),
        )
        .collect();
    let byte_to_line =
        |byte: usize| -> usize { line_starts.partition_point(|&start| start <= byte) };

    let mut prev_line: Option<usize> = None;
    let mut prev_end_byte: usize = 0;
    let mut line_buf = String::new();
    let mut line_start_num: usize = 0;
    let mut line_end_num: usize = 0;
    let mut line_has_loc = false;

    let flush_line = |buf: &mut String,
                      w: &mut &mut dyn Write,
                      has_loc: bool,
                      start: usize,
                      end: usize|
     -> io::Result<()> {
        let trimmed = buf.trim_end();
        write!(w, "{}", trimmed)?;
        if has_loc {
            if end > start {
                write!(w, " // L{}-L{}", start, end)?;
            } else {
                write!(w, " // L{}", start)?;
            }
        }
        writeln!(w)?;
        buf.clear();
        Ok(())
    };

    for range in ranges {
        let text = &source[range.start_byte..range.end_byte];
        if text.trim().is_empty() {
            continue;
        }

        // Skip leading whitespace/newlines to find actual content start
        let trimmed_offset = text.len() - text.trim_start().len();
        let content_start_byte = range.start_byte + trimmed_offset;
        let start_line = byte_to_line(content_start_byte);
        let new_line = prev_line.map_or(true, |prev| start_line > prev);

        if new_line {
            if prev_line.is_some() {
                flush_line(
                    &mut line_buf,
                    &mut (w as &mut dyn Write),
                    line_has_loc,
                    line_start_num,
                    line_end_num,
                )?;
            }
            let line_start = if start_line > 1 {
                line_starts[start_line - 1]
            } else {
                0
            };
            let line_text = &source[line_start..];
            let indent_len = line_text.len() - line_text.trim_start().len();
            line_buf.push_str(prefix);
            line_buf.push_str(&source[line_start..line_start + indent_len]);
            line_buf.push_str(text.trim_start());
            line_start_num = start_line;
            line_end_num = start_line;
            line_has_loc = !range.noloc;
        } else {
            if range.start_byte > prev_end_byte {
                let gap = &source[prev_end_byte..range.start_byte];
                if !gap.contains('\n') {
                    line_buf.push_str(gap);
                }
            }
            line_buf.push_str(text);
        }

        if !range.noloc {
            line_has_loc = true;
        }
        prev_end_byte = range.end_byte;
        let last_byte = range.start_byte + text.len();
        let end_line = byte_to_line(last_byte.saturating_sub(1));
        if end_line > line_end_num {
            line_end_num = end_line;
        }
        prev_line = Some(end_line);
    }

    if prev_line.is_some() {
        let trimmed = line_buf.trim_end();
        write!(w, "{}", trimmed)?;
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

struct CaptureNode<'a> {
    node: Node<'a>,
    is_show: bool,
    is_hide: bool,
    noloc: bool,
    show_after: bool,
    hide_after: bool,
    show_if_ref: bool,
    referenced: bool,
    name: Option<String>,
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

    // Phase 1: Collect all captures into a flat map keyed by node ID
    let mut captures: HashMap<usize, CaptureNode> = HashMap::new();
    let mut ref_texts: Vec<String> = Vec::new();

    while let Some(m) = matches.next() {
        let mut match_name: Option<String> = None;
        let mut match_show_if_ref_id: Option<usize> = None;

        for cap in m.captures {
            let capture_name: &str = &query.capture_names()[cap.index as usize];
            let node = cap.node;

            if capture_name == "name" {
                match_name = Some(source[node.byte_range()].trim().to_string());
                continue;
            }

            if capture_name == "ref" {
                ref_texts.push(source[node.byte_range()].trim().to_string());
                continue;
            }

            let parts: std::collections::HashSet<&str> = capture_name.split('.').collect();
            let is_show = parts.contains("show");
            let is_hide = parts.contains("hide");
            let is_show_if_ref = parts.contains("show_if_ref");
            let is_show_after = parts.contains("show_after");
            let is_hide_after = parts.contains("hide_after");

            if !is_show && !is_hide && !is_show_if_ref && !is_show_after && !is_hide_after {
                continue;
            }

            if is_show_if_ref {
                match_show_if_ref_id = Some(node.id());
            }

            captures.entry(node.id()).or_insert(CaptureNode {
                node,
                is_show: is_show || is_show_if_ref,
                is_hide,
                noloc: parts.contains("noloc"),
                show_after: is_show_after,
                hide_after: is_hide_after,
                show_if_ref: is_show_if_ref,
                referenced: false,
                name: None,
            });
        }

        if let Some(name) = match_name {
            if let Some(id) = match_show_if_ref_id {
                if let Some(cap) = captures.get_mut(&id) {
                    cap.name = Some(name);
                }
            }
        }
    }

    // Phase 2: Build tree — stack-based, O(C log C)
    let mut sorted_ids: Vec<usize> = captures.keys().copied().collect();
    sorted_ids.sort_by(|&a, &b| {
        let (ca, cb) = (&captures[&a], &captures[&b]);
        ca.node
            .start_byte()
            .cmp(&cb.node.start_byte())
            .then(cb.node.end_byte().cmp(&ca.node.end_byte()))
    });

    let mut children_map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut root_ids: Vec<usize> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    for &id in &sorted_ids {
        let start = captures[&id].node.start_byte();
        while let Some(&top) = stack.last() {
            if captures[&top].node.end_byte() > start {
                break;
            }
            stack.pop();
        }
        if let Some(&parent) = stack.last() {
            children_map.entry(parent).or_default().push(id);
        } else {
            root_ids.push(id);
        }
        stack.push(id);
    }

    // Phase 3: Apply @ref — mark matching @show_if_ref nodes as referenced
    for ref_text in &ref_texts {
        for cap in captures.values_mut() {
            if cap.name.as_deref() == Some(ref_text.as_str()) {
                cap.referenced = true;
                break;
            }
        }
    }

    // Phase 4: Walk tree to generate visible ranges
    let mut ranges = Vec::new();
    for &id in &root_ids {
        emit_ranges(id, source, &captures, &children_map, &mut ranges);
    }

    ranges
}

fn emit_ranges(
    node_id: usize,
    source: &str,
    captures: &HashMap<usize, CaptureNode>,
    children_map: &HashMap<usize, Vec<usize>>,
    output: &mut Vec<VisibleRange>,
) {
    let cap = &captures[&node_id];
    let children = children_map
        .get(&node_id)
        .map(|v| v.as_slice())
        .unwrap_or(&[]);

    // Hide node: recurse into children with sibling visibility toggle
    if cap.is_hide {
        let mut hidden = false;
        for &child_id in children {
            let child = &captures[&child_id];
            if child.show_after {
                hidden = false;
            }
            if child.hide_after {
                hidden = true;
            }
            if !hidden {
                emit_ranges(child_id, source, captures, children_map, output);
            }
        }
        return;
    }

    // Toggle-only node (show_after/hide_after without show): nothing to emit
    if !cap.is_show {
        return;
    }

    // Unreferenced show_if_ref: skip
    if cap.show_if_ref && !cap.referenced {
        return;
    }

    // Show node: emit own byte range minus children's ranges
    let mut pos = cap.node.start_byte();
    let end = cap.node.end_byte();

    for &child_id in children {
        let child = &captures[&child_id];
        let cs = child.node.start_byte();
        let ce = child.node.end_byte();

        if cs > pos {
            // Trim trailing indentation after the last newline in the gap,
            // so it doesn't merge with the next line's content.
            let gap = &source[pos..cs];
            let gap_end = if let Some(last_nl) = gap.rfind('\n') {
                let after_nl = &gap[last_nl + 1..];
                if after_nl.bytes().all(|b| b == b' ' || b == b'\t') {
                    pos + last_nl + 1
                } else {
                    cs
                }
            } else {
                cs
            };
            if gap_end > pos {
                output.push(VisibleRange {
                    start_byte: pos,
                    end_byte: gap_end,
                    noloc: cap.noloc,
                });
            }
        }
        emit_ranges(child_id, source, captures, children_map, output);
        pos = pos.max(ce);
    }

    if pos < end {
        output.push(VisibleRange {
            start_byte: pos,
            end_byte: end,
            noloc: cap.noloc,
        });
    }
}
