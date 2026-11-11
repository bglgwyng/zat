use std::process::Command;

fn zat() -> Command {
    Command::new(env!("CARGO_BIN_EXE_zat"))
}

#[test]
fn help_exits_zero() {
    for flag in ["--help", "-h"] {
        let out = zat().arg(flag).output().expect("spawn zat");
        assert_eq!(out.status.code(), Some(0), "flag {flag}");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("USAGE"), "flag {flag}");
        assert!(stdout.contains("--lang"), "flag {flag}");
        assert!(stdout.contains("rust"), "flag {flag}");
    }
}

#[test]
fn version_exits_zero() {
    for flag in ["--version", "-V"] {
        let out = zat().arg(flag).output().expect("spawn zat");
        assert_eq!(out.status.code(), Some(0), "flag {flag}");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            stdout.trim() == format!("zat {}", env!("CARGO_PKG_VERSION")),
            "flag {flag} got {stdout:?}"
        );
    }
}

#[test]
fn no_args_exits_one() {
    let out = zat().output().expect("spawn zat");
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn unknown_option_exits_two() {
    let out = zat().arg("--wat").output().expect("spawn zat");
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown option"), "got {stderr:?}");
}

#[test]
fn unknown_language_exits_two() {
    let out = zat()
        .args(["-l", "bogus", "src/main.rs"])
        .output()
        .expect("spawn zat");
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown language"), "got {stderr:?}");
}

#[test]
fn missing_value_exits_two() {
    let out = zat().arg("-l").output().expect("spawn zat");
    assert_eq!(out.status.code(), Some(2));
}

#[test]
fn lang_matches_extension_inference() {
    // Every supported language: canonical name, an alias, and a fixture extension.
    let cases: &[(&str, &str, &str)] = &[
        ("c", "c", "c"),
        ("cpp", "cpp", "cpp"),
        ("csharp", "cs", "cs"),
        ("go", "go", "go"),
        ("haskell", "hs", "hs"),
        ("java", "java", "java"),
        ("javascript", "js", "js"),
        ("kotlin", "kt", "kt"),
        ("markdown", "md", "md"),
        ("python", "py", "py"),
        ("ruby", "rb", "rb"),
        ("rust", "rs", "rs"),
        ("swift", "swift", "swift"),
        ("typescript", "ts", "ts"),
    ];

    for (lang, alias, ext) in cases {
        let fixture = format!("tests/fixtures/sample.{ext}");

        let auto = zat()
            .arg(&fixture)
            .output()
            .unwrap_or_else(|e| panic!("auto for {ext}: {e}"));
        assert_eq!(
            auto.status.code(),
            Some(0),
            "auto for {ext}: {}",
            String::from_utf8_lossy(&auto.stderr)
        );

        for (label, args) in [
            ("--lang <name>", vec!["--lang".to_string(), lang.to_string()]),
            ("--lang <alias>", vec!["--lang".to_string(), alias.to_string()]),
            ("-l <alias>", vec!["-l".to_string(), alias.to_string()]),
        ] {
            let forced = zat().args(&args).arg(&fixture).output().expect("spawn zat");
            assert_eq!(
                forced.status.code(),
                Some(0),
                "{label} for {ext}: {}",
                String::from_utf8_lossy(&forced.stderr)
            );
            assert_eq!(
                auto.stdout,
                forced.stdout,
                "{label} differs from extension inference for {ext}"
            );
        }
    }
}

#[test]
fn extension_inference_is_overridden() {
    // Force C++ onto a Rust fixture: output must differ from Rust inference,
    // proving --lang actually overrides the extension.
    let fixture = "tests/fixtures/sample.rs";

    let as_rust = zat().arg(fixture).output().expect("spawn zat");
    let as_cpp = zat()
        .args(["-l", "cpp", fixture])
        .output()
        .expect("spawn zat");

    assert_eq!(as_rust.status.code(), Some(0));
    assert_eq!(as_cpp.status.code(), Some(0));
    assert_ne!(as_rust.stdout, as_cpp.stdout);
}