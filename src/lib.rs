pub mod outline;

use tree_sitter::Language;

/// A single supported language: its grammar name, query source, canonical
/// display name, and the file extensions that map to it.
struct LanguageSpec {
    name: &'static str,
    query_src: &'static str,
    extensions: &'static [&'static str],
}

const TABLE: &[LanguageSpec] = &[
    LanguageSpec {
        name: "c",
        query_src: include_str!("../queries/c.scm"),
        extensions: &["c", "h"],
    },
    LanguageSpec {
        name: "cpp",
        query_src: include_str!("../queries/cpp.scm"),
        extensions: &["cpp", "cc", "cxx", "hpp", "hxx"],
    },
    LanguageSpec {
        name: "csharp",
        query_src: include_str!("../queries/csharp.scm"),
        extensions: &["cs"],
    },
    LanguageSpec {
        name: "go",
        query_src: include_str!("../queries/go.scm"),
        extensions: &["go"],
    },
    LanguageSpec {
        name: "haskell",
        query_src: include_str!("../queries/haskell.scm"),
        extensions: &["hs"],
    },
    LanguageSpec {
        name: "java",
        query_src: include_str!("../queries/java.scm"),
        extensions: &["java"],
    },
    LanguageSpec {
        name: "javascript",
        query_src: include_str!("../queries/javascript.scm"),
        extensions: &["js", "jsx", "cjs", "mjs"],
    },
    LanguageSpec {
        name: "kotlin",
        query_src: include_str!("../queries/kotlin.scm"),
        extensions: &["kt", "kts"],
    },
    LanguageSpec {
        name: "markdown",
        query_src: include_str!("../queries/markdown.scm"),
        extensions: &["md", "markdown"],
    },
    LanguageSpec {
        name: "python",
        query_src: include_str!("../queries/python.scm"),
        extensions: &["py"],
    },
    LanguageSpec {
        name: "ruby",
        query_src: include_str!("../queries/ruby.scm"),
        extensions: &["rb"],
    },
    LanguageSpec {
        name: "rust",
        query_src: include_str!("../queries/rust.scm"),
        extensions: &["rs"],
    },
    LanguageSpec {
        name: "swift",
        query_src: include_str!("../queries/swift.scm"),
        extensions: &["swift"],
    },
    LanguageSpec {
        name: "typescript",
        query_src: include_str!("../queries/typescript.scm"),
        extensions: &["ts", "mts", "cts"],
    },
    LanguageSpec {
        name: "tsx",
        query_src: include_str!("../queries/typescript.scm"),
        extensions: &["tsx"],
    },
];

fn language_for(spec: &LanguageSpec) -> Language {
    match spec.name {
        "c" => tree_sitter_c::LANGUAGE.into(),
        "cpp" => tree_sitter_cpp::LANGUAGE.into(),
        "csharp" => tree_sitter_c_sharp::LANGUAGE.into(),
        "go" => tree_sitter_go::LANGUAGE.into(),
        "haskell" => tree_sitter_haskell::LANGUAGE.into(),
        "java" => tree_sitter_java::LANGUAGE.into(),
        "javascript" => tree_sitter_javascript::LANGUAGE.into(),
        "kotlin" => tree_sitter_kotlin_ng::LANGUAGE.into(),
        "markdown" => tree_sitter_md::LANGUAGE.into(),
        "python" => tree_sitter_python::LANGUAGE.into(),
        "ruby" => tree_sitter_ruby::LANGUAGE.into(),
        "rust" => tree_sitter_rust::LANGUAGE.into(),
        "swift" => tree_sitter_swift::LANGUAGE.into(),
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX.into(),
        _ => unreachable!("unknown language spec: {}", spec.name),
    }
}

/// Resolve a language from a file extension.
///
/// The extension is matched case-insensitively and without a leading dot.
pub fn lang_for_ext(ext: &str) -> Option<(Language, &'static str)> {
    let ext = ext.trim_start_matches('.').to_ascii_lowercase();
    TABLE
        .iter()
        .find(|spec| spec.extensions.contains(&ext.as_str()))
        .map(|spec| (language_for(spec), spec.query_src))
}

/// Resolve a language from a user-supplied name.
///
/// Accepts the canonical language name (e.g. `rust`, `python`) or any of the
/// file extensions it maps to (e.g. `rs`, `py`). Matching is case-insensitive
/// and a leading dot is ignored, so `--lang .RS` works.
pub fn lang_for_name(name: &str) -> Option<(Language, &'static str)> {
    let name = name.trim_start_matches('.').to_ascii_lowercase();
    TABLE
        .iter()
        .find(|spec| {
            spec.name.eq_ignore_ascii_case(&name)
                || spec.extensions.contains(&name.as_str())
        })
        .map(|spec| (language_for(spec), spec.query_src))
}

/// The canonical names of every supported language, in display order.
pub fn supported_languages() -> &'static [&'static str] {
    const NAMES: &[&str] = &[
        "c",
        "cpp",
        "csharp",
        "go",
        "haskell",
        "java",
        "javascript",
        "kotlin",
        "markdown",
        "python",
        "ruby",
        "rust",
        "swift",
        "typescript",
        "tsx",
    ];
    NAMES
}