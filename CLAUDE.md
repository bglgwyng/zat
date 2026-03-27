# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`zat` is a single Rust binary that shows code outlines (exported symbols with line numbers) for 13 languages. For directories, it finds entry files and shows their outlines alongside a file listing.

## Build & Test

```bash
cargo build                  # Dev build
cargo build --release        # Release build
nix build                    # Build via Nix (uses flake.nix + buildRustPackage)
cargo test                   # Run all snapshot tests
cargo test test_rust         # Run a single language test
```

## Release

Tag with `v*` triggers `.github/workflows/release.yml` which:
1. Builds on 3 platforms (aarch64-darwin, x86_64-darwin, x86_64-linux) via `cargo build`
2. Creates a GitHub Release with tarballs
3. Pushes a Homebrew formula to `bglgwyng/homebrew-tap`

## Architecture

- **`src/main.rs`**: Entry point. Handles file vs directory dispatch, extension→language mapping, and `cat -n` fallback. All tree-sitter queries are `include_str!`'d at compile time.

- **`src/outline.rs`**: Core outline extraction engine. Takes source code, a tree-sitter `Language`, and a query string; returns `Vec<VisibleRange>`. Uses `node.parent()` to assign `@hide` ranges to their containing `@show` nodes, then walks the AST tree to collect visible byte ranges.

- **`queries/*.scm`**: Tree-sitter query files per language. These define what appears in outlines using a capture-based system:
  - `@show` — symbol to display (source indentation is preserved automatically)
  - `@hide` — range to omit within a `@show` node (e.g. function body, `pub` modifier)
  - `@show_if_ref` / `@name` / `@ref` — conditional display for re-exported symbols
  - `@show_after` / `@hide_after` — sibling visibility toggles (e.g. C++ access specifiers)
  - `.noloc` modifier — suppress line numbers

- **`src/lib.rs`**: `lang_for_ext()` maps file extensions to `(Language, query_src)` pairs. This is the central registry for supported languages.

- **`flake.nix`**: Nix build config using `rustPlatform.buildRustPackage` with `rust-overlay`.

## Testing

Tests are snapshot-based. Each language has a fixture (`tests/fixtures/sample.{ext}`) and a snapshot (`tests/snapshots/sample.{ext}.snap`). The test runs the outline extractor on the fixture and compares output to the snapshot. To update a snapshot after changing a query, run the outline on the fixture and write the output to the snapshot file.

## Adding a Language

1. Add `tree-sitter-{lang}` dependency to `Cargo.toml`
2. Create `queries/{lang}.scm` with `@show`/`@hide` captures
3. Add extension mapping in `src/lib.rs` (`lang_for_ext`)
4. Add test fixture `tests/fixtures/sample.{ext}` and snapshot `tests/snapshots/sample.{ext}.snap`
5. Add test function in `tests/outline_test.rs`
