pub mod outline;

use tree_sitter::Language;

pub fn lang_for_ext(ext: &str) -> Option<(Language, &'static str)> {
    Some(match ext {
        "c" | "h" => (
            tree_sitter_c::LANGUAGE.into(),
            include_str!("../queries/c.scm"),
        ),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" => (
            tree_sitter_cpp::LANGUAGE.into(),
            include_str!("../queries/cpp.scm"),
        ),
        "cs" => (
            tree_sitter_c_sharp::LANGUAGE.into(),
            include_str!("../queries/csharp.scm"),
        ),
        "go" => (
            tree_sitter_go::LANGUAGE.into(),
            include_str!("../queries/go.scm"),
        ),
        "hs" => (
            tree_sitter_haskell::LANGUAGE.into(),
            include_str!("../queries/haskell.scm"),
        ),
        "java" => (
            tree_sitter_java::LANGUAGE.into(),
            include_str!("../queries/java.scm"),
        ),
        "js" | "jsx" | "cjs" | "mjs" => (
            tree_sitter_javascript::LANGUAGE.into(),
            include_str!("../queries/javascript.scm"),
        ),
        "kt" | "kts" => (
            tree_sitter_kotlin_ng::LANGUAGE.into(),
            include_str!("../queries/kotlin.scm"),
        ),
        "md" | "markdown" => (
            tree_sitter_md::LANGUAGE.into(),
            include_str!("../queries/markdown.scm"),
        ),
        "py" => (
            tree_sitter_python::LANGUAGE.into(),
            include_str!("../queries/python.scm"),
        ),
        "rb" => (
            tree_sitter_ruby::LANGUAGE.into(),
            include_str!("../queries/ruby.scm"),
        ),
        "rs" => (
            tree_sitter_rust::LANGUAGE.into(),
            include_str!("../queries/rust.scm"),
        ),
        "swift" => (
            tree_sitter_swift::LANGUAGE.into(),
            include_str!("../queries/swift.scm"),
        ),
        "ts" | "mts" | "cts" => (
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            include_str!("../queries/typescript.scm"),
        ),
        "tsx" => (
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            include_str!("../queries/typescript.scm"),
        ),
        _ => return None,
    })
}
