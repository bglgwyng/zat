use std::collections::HashMap;
use std::io::{self, Write};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor, Tree};

enum Visibility {
    Show,
    Hide,
    ShowIfRef,
}

pub struct VisibleRange<'a> {
    pub node: Node<'a>,
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_row: usize,
    pub noloc: bool,
}

/// Walk up then left to find the line's leading whitespace extent.
fn get_indent(node: &Node) -> (usize, usize) {
    let row = node.start_position().row;
    let mut current = *node;
    while let Some(parent) = current.parent() {
        if parent.start_position().row != row {
            break;
        }
        current = parent;
    }
    while let Some(prev) = current.prev_sibling() {
        if prev.start_position().row != row {
            break;
        }
        current = prev;
    }
    let col = current.start_position().column;
    let line_start = current.start_byte() - col;
    (line_start, col)
}

pub fn write_outline(
    source: &str,
    ranges: &[VisibleRange<'_>],
    prefix: &str,
    w: &mut impl Write,
) -> io::Result<()> {
    let mut prev_row: Option<usize> = None;
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

        let leading_newlines = text
            .bytes()
            .take_while(|b| b.is_ascii_whitespace())
            .filter(|&b| b == b'\n')
            .count();
        let content_row = range.start_row + leading_newlines;
        let new_line = prev_row.map_or(true, |prev| content_row > prev);

        if new_line {
            if prev_row.is_some() {
                flush_line(
                    &mut line_buf,
                    &mut (w as &mut dyn Write),
                    line_has_loc,
                    line_start_num,
                    line_end_num,
                )?;
            }
            let (indent_byte, indent_len) = get_indent(&range.node);
            line_buf.push_str(prefix);
            line_buf.push_str(&source[indent_byte..indent_byte + indent_len]);
            line_buf.push_str(text.trim_start());
            line_start_num = content_row + 1;
            line_end_num = content_row + 1;
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
        let content_newlines = text.trim().bytes().filter(|&b| b == b'\n').count();
        let end_row = content_row + content_newlines;
        let end_line = end_row + 1;
        if end_line > line_end_num {
            line_end_num = end_line;
        }
        prev_row = Some(end_row);
    }

    if prev_row.is_some() {
        flush_line(
            &mut line_buf,
            &mut (w as &mut dyn Write),
            line_has_loc,
            line_start_num,
            line_end_num,
        )?;
    }

    Ok(())
}

struct CaptureNode<'a> {
    node: Node<'a>,
    visibility: Option<Visibility>,
    noloc: bool,
    show_after: bool,
    hide_after: bool,
    referenced: bool,
    name: Option<String>,
    children_ids: Vec<usize>,
}

pub fn parse(source: &str, language: &Language) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Failed to set language");
    parser.parse(source, None)
}

pub fn extract_outline<'a>(
    source: &str,
    tree: &'a Tree,
    language: &Language,
    query_src: &str,
) -> Vec<VisibleRange<'a>> {
    let query = match Query::new(language, query_src) {
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
            if parts.is_empty() {
                continue;
            }

            let visibility = if parts.contains("show") {
                Some(Visibility::Show)
            } else if parts.contains("show_if_ref") {
                match_show_if_ref_id = Some(node.id());
                Some(Visibility::ShowIfRef)
            } else if parts.contains("hide") {
                Some(Visibility::Hide)
            } else {
                None
            };

            let is_show_after = parts.contains("show_after");
            let is_hide_after = parts.contains("hide_after");

            captures.entry(node.id()).or_insert(CaptureNode {
                node,
                visibility,
                noloc: parts.contains("noloc"),
                show_after: is_show_after,
                hide_after: is_hide_after,
                referenced: false,
                name: None,
                children_ids: Vec::new(),
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
            // Same byte range: more descendants = ancestor, should come first
            .then(cb.node.descendant_count().cmp(&ca.node.descendant_count()))
    });

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
            captures.get_mut(&parent).unwrap().children_ids.push(id);
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
        emit_ranges(id, &captures, &mut ranges);
    }

    ranges
}

fn emit_ranges<'a>(
    node_id: usize,
    captures: &HashMap<usize, CaptureNode<'a>>,
    output: &mut Vec<VisibleRange<'a>>,
) {
    let cap = &captures[&node_id];

    // Hide node: recurse into children with sibling visibility toggle
    match cap.visibility {
        Some(Visibility::ShowIfRef) if !cap.referenced => return,
        Some(Visibility::Hide) => {
            let mut hidden = false;
            for &child_id in cap.children_ids.iter() {
                let child = &captures[&child_id];
                if child.show_after {
                    hidden = false;
                }
                if child.hide_after {
                    hidden = true;
                }
                if !hidden {
                    emit_ranges(child_id, captures, output);
                }
            }
            return;
        }
        _ => {}
    }

    // Show node: emit own byte range minus children's ranges
    let mut row = cap.node.start_position().row;
    let mut start = cap.node.start_byte();
    let end = cap.node.end_byte();

    for &child_id in cap.children_ids.iter() {
        let child = &captures[&child_id];
        let cs = child.node.start_byte();
        let ce = child.node.end_byte();

        if cs > start {
            output.push(VisibleRange {
                node: cap.node,
                start_byte: start,
                end_byte: cs,
                start_row: row,
                noloc: cap.noloc,
            });
        }
        emit_ranges(child_id, captures, output);
        start = ce;
        row = child.node.end_position().row;
    }

    if start < end {
        output.push(VisibleRange {
            node: cap.node,
            start_byte: start,
            end_byte: end,
            start_row: row,
            noloc: cap.noloc,
        });
    }
}
