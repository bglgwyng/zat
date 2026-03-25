mod outline;

use std::fs;
use std::io::Write;
use std::path::Path;
use tree_sitter::Language;

use outline::{OutlineEntry, extract_outline};

fn lang_for_ext(ext: &str) -> Option<(Language, &'static str)> {
    Some(match ext {
        "go" => (
            tree_sitter_go::LANGUAGE.into(),
            include_str!("../queries/go.scm"),
        ),
        "c" | "h" => (
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../queries/c.scm"),
        ),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => (
            tree_sitter_cpp::LANGUAGE.into(),
            include_str!("../queries/cpp.scm"),
        ),
        "js" | "jsx" | "cjs" | "mjs" => (
            tree_sitter_javascript::LANGUAGE.into(),
            include_str!("../queries/javascript.scm"),
        ),
        "ts" | "mts" | "cts" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../queries/typescript.scm"),
        ),
        "tsx" => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../queries/typescript.scm"),
        ),
        "py" => (
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python.scm"),
        ),
        "rs" => (
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../queries/rust.scm"),
        ),
        "java" => (
            tree_sitter_java::LANGUAGE.into(),
            include_str!("../queries/java.scm"),
        ),
        "hs" => (
            tree_sitter_haskell::LANGUAGE.into(),
            include_str!("../queries/haskell.scm"),
        ),
        "swift" => (
            tree_sitter_swift::LANGUAGE.into(),
            include_str!("../queries/swift.scm"),
        ),
        "kt" | "kts" => (
            tree_sitter_kotlin_ng::LANGUAGE.into(),
            include_str!("../queries/kotlin.scm"),
        ),
        "cs" => (
            tree_sitter_c_sharp::LANGUAGE.into(),
            include_str!("../queries/csharp.scm"),
        ),
        "rb" => (
            tree_sitter_ruby::LANGUAGE.into(),
            include_str!("../queries/ruby.scm"),
        ),
        _ => return None,
    })
}

fn print_entries(source: &str, entries: &[OutlineEntry], prefix: &str) {
    let mut out = std::io::stdout().lock();
    for entry in entries {
        let _ = write!(out, "{}", prefix);
        let _ = entry.write_to(source, &mut out);
        if entry.start_line > 0 && !entry.noloc {
            if entry.end_line > entry.start_line {
                let _ = writeln!(out, " // L{}-L{}", entry.start_line, entry.end_line);
            } else {
                let _ = writeln!(out, " // L{}", entry.start_line);
            }
        } else {
            let _ = writeln!(out);
        }
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
            let entries = extract_outline(&source, language, query_src);
            print_entries(&source, &entries, "");
        }
        None => eprintln!("zat: {}: unknown file type", path.display()),
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
        eprintln!("zat: {}: is a directory", path.display());
        std::process::exit(1);
    } else {
        view_file(path);
    }
}
