use std::fs;
use zat::lang_for_ext;
use zat::outline::{extract_outline, parse, write_outline};

fn run_outline(ext: &str) -> String {
    let fixture_path = format!("tests/fixtures/sample.{ext}");
    let source = fs::read_to_string(&fixture_path)
        .unwrap_or_else(|e| panic!("Failed to read {fixture_path}: {e}"));

    let (language, query_src) =
        lang_for_ext(ext).unwrap_or_else(|| panic!("No language for extension: {ext}"));

    let tree = parse(&source, &language).expect("parse failed");
    let ranges = extract_outline(&source, &tree, &language, query_src);
    let mut buf = Vec::new();
    write_outline(&source, &ranges, "", &mut buf).expect("write_outline failed");
    String::from_utf8(buf).expect("non-UTF8 output")
}

fn assert_snapshot(ext: &str) {
    let actual = run_outline(ext);
    let snap_path = format!("tests/snapshots/sample.{ext}.snap");
    let expected = fs::read_to_string(&snap_path)
        .unwrap_or_else(|e| panic!("Failed to read {snap_path}: {e}"));
    assert_eq!(actual, expected, "Snapshot mismatch for {ext}");
}

#[test]
fn test_rust() {
    assert_snapshot("rs");
}

#[test]
fn test_go() {
    assert_snapshot("go");
}

#[test]
fn test_python() {
    assert_snapshot("py");
}

#[test]
fn test_typescript() {
    assert_snapshot("ts");
}

#[test]
fn test_javascript() {
    assert_snapshot("js");
}

#[test]
fn test_java() {
    assert_snapshot("java");
}

#[test]
fn test_c() {
    assert_snapshot("c");
}

#[test]
fn test_cpp() {
    assert_snapshot("cpp");
}

#[test]
fn test_csharp() {
    assert_snapshot("cs");
}

#[test]
fn test_haskell() {
    assert_snapshot("hs");
}

#[test]
fn test_swift() {
    assert_snapshot("swift");
}

#[test]
fn test_kotlin() {
    assert_snapshot("kt");
}

#[test]
fn test_ruby() {
    assert_snapshot("rb");
}

#[test]
fn test_markdown() {
    assert_snapshot("md");
}
