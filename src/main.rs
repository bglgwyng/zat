mod outline;

use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tree_sitter::Language;

use outline::{extract_outline, OutlineEntry};

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

fn print_entries(entries: &[OutlineEntry], prefix: &str) {
    for entry in entries {
        print!("{}{}", prefix, entry.text);
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
            let entries = extract_outline(&source, language, query_src);
            print_entries(&entries, "");
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
            let ext = target.extension().and_then(|e| e.to_str()).unwrap_or("");
            if let Some((language, query_src)) = lang_for_ext(ext) {
                let source = fs::read_to_string(&target).unwrap_or_default();
                let entries = extract_outline(&source, language, query_src);
                print_entries(&entries, "  ");
            } else {
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
