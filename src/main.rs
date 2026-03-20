use std::collections::BTreeMap;
use std::fs;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

const ENTRY_FILES: &[&str] = &[
    "index.md",
    "README.md",
    "index.ts",
    "index.js",
    "index.tsx",
    "index.jsx",
    "mod.rs",
    "lib.rs",
    "main.rs",
    "__init__.py",
];

fn lang_for_ext(ext: &str) -> Option<(Language, &'static str)> {
    Some(match ext {
        "go" => (tree_sitter_go::LANGUAGE.into(), include_str!("../queries/go.scm")),
        "c" | "h" => (tree_sitter_c::LANGUAGE.into(), include_str!("../queries/c.scm")),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => (tree_sitter_cpp::LANGUAGE.into(), include_str!("../queries/cpp.scm")),
        "js" | "jsx" | "cjs" | "mjs" => (tree_sitter_javascript::LANGUAGE.into(), include_str!("../queries/javascript.scm")),
        "ts" | "mts" | "cts" => (tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), include_str!("../queries/typescript.scm")),
        "tsx" => (tree_sitter_typescript::LANGUAGE_TSX.into(), include_str!("../queries/typescript.scm")),
        "py" => (tree_sitter_python::LANGUAGE.into(), include_str!("../queries/python.scm")),
        "rs" => (tree_sitter_rust::LANGUAGE.into(), include_str!("../queries/rust.scm")),
        "java" => (tree_sitter_java::LANGUAGE.into(), include_str!("../queries/java.scm")),
        "hs" => (tree_sitter_haskell::LANGUAGE.into(), include_str!("../queries/haskell.scm")),
        "swift" => (tree_sitter_swift::LANGUAGE.into(), include_str!("../queries/swift.scm")),
        "kt" | "kts" => (tree_sitter_kotlin_ng::LANGUAGE.into(), include_str!("../queries/kotlin.scm")),
        "cs" => (tree_sitter_c_sharp::LANGUAGE.into(), include_str!("../queries/csharp.scm")),
        "rb" => (tree_sitter_ruby::LANGUAGE.into(), include_str!("../queries/ruby.scm")),
        _ => return None,
    })
}

fn print_outline(source: &str, language: Language, query_src: &str) {
    let entries = extract_outline(source, language, query_src);
    for entry in &entries {
        print!("{}", entry.text);
        if entry.start_line > 0 && !entry.noloc {
            if entry.end_line > entry.start_line {
                println!(" // L{}-L{}", entry.start_line, entry.end_line);
            } else {
                println!(" // L{}", entry.start_line);
            }
        } else {
            println!();
        }
    }
}

fn print_fallback(path: &Path) {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("zat: {}: {}", path.display(), e);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_default();
        println!("{:>6}\t{}", i + 1, line);
    }
}

fn view_file(path: &Path) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    match lang_for_ext(ext) {
        Some((language, query_src)) => {
            let source = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("zat: {}: {}", path.display(), e);
                std::process::exit(1);
            });
            print_outline(&source, language, query_src);
        }
        None => print_fallback(path),
    }
}

fn view_directory(dir: &Path) {
    let mut printed = false;

    for entry_name in ENTRY_FILES {
        let target = dir.join(entry_name);
        if target.is_file() {
            if printed {
                println!();
            }
            println!("{}:", entry_name);
            // Capture outline output with indentation
            let ext = target.extension().and_then(|e| e.to_str()).unwrap_or("");
            if let Some((language, query_src)) = lang_for_ext(ext) {
                let source = fs::read_to_string(&target).unwrap_or_default();
                let entries = extract_outline(&source, language, query_src);
                for entry in &entries {
                    print!("  {}", entry.text);
                    if entry.start_line > 0 && !entry.noloc {
                        if entry.end_line > entry.start_line {
                            println!(" // L{}-L{}", entry.start_line, entry.end_line);
                        } else {
                            println!(" // L{}", entry.start_line);
                        }
                    } else {
                        println!();
                    }
                }
            } else {
                // Fallback for non-outline files (e.g. index.md)
                let content = fs::read_to_string(&target).unwrap_or_default();
                for (i, line) in content.lines().enumerate() {
                    println!("  {:>6}\t{}", i + 1, line);
                }
            }
            printed = true;
        }
    }

    // List directory contents
    if let Ok(mut entries) = fs::read_dir(dir) {
        let mut names: Vec<String> = Vec::new();
        while let Some(Ok(entry)) = entries.next() {
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
        names.sort();
        if !names.is_empty() {
            if printed {
                println!();
            }
            println!(".:");
            for name in &names {
                println!("  {}", name);
            }
        }
    }
}

fn main() {
    let path_arg = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: zat <file-or-directory>");
        std::process::exit(1);
    });

    let path = Path::new(&path_arg);
    if !path.exists() {
        eprintln!("zat: {}: No such file or directory", path.display());
        std::process::exit(1);
    }

    if path.is_dir() {
        view_directory(path);
    } else {
        view_file(path);
    }
}

struct OutlineEntry {
    text: String,
    start_line: usize,
    end_line: usize,
    noloc: bool,
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
