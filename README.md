# zat

`zat` is a code outline viewer. It shows exported symbols and line numbers at a glance.

Works great as a tool for LLM coding agents — they can see the outline first, then `Read` only the parts they need.

The name comes from Japanese ざっと (*zatto*), meaning "roughly" or "at a glance".

## Installation

### Homebrew

```shell
brew install bglgwyng/tap/zat
```

### Nix

```shell
nix profile install github:bglgwyng/zat
```

Or run directly:

```shell
nix run github:bglgwyng/zat -- <FILE>
```

### Cargo

```shell
cargo install --git https://github.com/bglgwyng/zat
```

### Pre-built binaries

Download from [GitHub Releases](https://github.com/bglgwyng/zat/releases). Available for macOS (aarch64, x86_64) and Linux (x86_64).

## Usage

### File outline

```shell
zat src/lib.rs
```

```
struct OutlineEntry { // L8-L12
  signature: String
  start_line: usize
  end_line: usize
}
struct ImportEntry { // L14-L18
  source_text: String
  start_line: usize
  end_line: usize
}
fn extract_outline(source: & str, path: & str) -> OutlineResult // L33-L196
```

Only public/exported symbols are shown. Visibility modifiers (`pub`, `export`) are stripped for brevity. Struct fields, enum variants, and interface members are included.

### Directory outline

```shell
zat src/
```

```
lib.rs:
  struct Config { // L5-L10
    name: String
    debug: bool
  }
  fn load(path: & str) -> Config // L12-L30

.:
  lib.rs
  utils.rs
```

Looks for entry files (`index.ts`, `lib.rs`, `__init__.py`, etc.) and shows their outlines, followed by a directory listing under `.:`.

## Supported Languages

| Language | Extensions |
|----------|-----------|
| JavaScript | `.js`, `.jsx`, `.cjs`, `.mjs` |
| TypeScript | `.ts`, `.tsx`, `.mts`, `.cts` |
| Rust | `.rs` |
| Python | `.py` |
| Go | `.go` |
| Java | `.java` |
| C | `.c`, `.h` |
| C++ | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx` |
| C# | `.cs` |
| Swift | `.swift` |
| Kotlin | `.kt`, `.kts` |
| Haskell | `.hs` |
| Ruby | `.rb` |
| Other | Falls back to `cat -n` |

All languages are parsed with [tree-sitter](https://tree-sitter.github.io/).

## For AI Agents

Add this to your `CLAUDE.md` or `AGENTS.md`:

````markdown
## Tools

### zat

A code outline viewer that shows exported symbol signatures with line numbers.

**Prefer `zat` over other tools for initial exploration:**
- **Over `cat` or `Read`** — when you need interfaces/signatures, not full implementation
- **Over `ls`** — when exploring a directory, to get richer context (entry file outlines + listing)

Use `zat` first to get an outline, then `Read(offset, limit)` to read only the relevant sections.

- `zat <file>` — Show outline (13 languages supported; falls back to `cat -n` for other types)
- `zat <dir>` — Show entry file outlines (index.ts, lib.rs, etc.) and directory listing
- Use the line numbers (e.g. `L10-L25`) to `Read(offset, limit)` into specific ranges
- Pipe through `head` to limit output: `zat <file> | head -n 50`
````
