# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`zat` is a single Rust binary that shows code outlines (exported symbols with line numbers) for 13 languages. For directories, it finds entry files and shows their outlines alongside a file listing.

## Build

```bash
nix build                    # Build via Nix (uses flake.nix + buildRustPackage)
cargo build --release        # Build directly with Cargo
```

## Release

Tag with `v*` triggers `.github/workflows/release.yml` which:
1. Builds on 3 platforms (aarch64-darwin, x86_64-darwin, x86_64-linux) via `cargo build`
2. Creates a GitHub Release with tarballs
3. Pushes a Homebrew formula to `bglgwyng/homebrew-tap`

## Architecture

- **`src/main.rs`**: Single entry point. Handles file vs directory dispatch, extension→language mapping, outline extraction, and `cat -n` fallback. All tree-sitter queries are `include_str!`'d at compile time.

- **`queries/*.scm`**: Tree-sitter query files per language. These define what appears in outlines using a capture-based system:
  - `@show` — top-level symbol to display
  - `@show.indented` / `@show.indent` — child member (e.g. struct field, method)
  - `@hide` — body/initializer to omit (shows first line only, plus closing delimiter)
  - `@strip` — token to remove from output (e.g. `pub`, `export`)
  - `@show_if_ref` / `@name` / `@ref` — conditional display for re-exported symbols
  - `@show_after` / `@hide_after` — visibility toggles within a block
  - `@append` — attach adjacent text to the show node
  - `.noloc` modifier — suppress line numbers

- **`flake.nix`**: Nix build config using `rustPlatform.buildRustPackage` with `rust-overlay`.
