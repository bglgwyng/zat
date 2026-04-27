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

[Available in nixpkgs](https://search.nixos.org/packages?show=zat).

```shell
nix profile install nixpkgs#zat
```

Or run directly:

```shell
nix run nixpkgs#zat -- <FILE>
```

Also available via [llm-agents.nix](https://github.com/numtide/llm-agents.nix).

#### Home Manager

Add `github:bglgwyng/zat` as a flake input, import `zat.homeManagerModules.default`, and configure:

```nix
{
  programs.zat = {
    enable = true;
    claude-code.enable = true;
  };
}
```

Options:

- `programs.zat.enable` — installs the `zat` binary
- `programs.zat.claude.enable` — registers a `zat` rule under `programs.claude-code.rules` (requires a `claude-code` Home Manager module that exposes that option)

### mise

```shell
mise use -g github:bglgwyng/zat
```

Or run directly:

```shell
mise x github:bglgwyng/zat -- zat <FILE>
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

All languages are parsed with [tree-sitter](https://tree-sitter.github.io/).

## For AI Agents

Add this to your `CLAUDE.md` or `AGENTS.md`:

````markdown
## Tools

### zat

A code outline viewer that shows exported symbol signatures with line numbers.

Prefer `zat` over `cat`/`Read` when you need signatures, not full implementation. Use the line numbers in the output to `Read(offset, limit)` into specific sections.

Supported languages: C, C++, C#, Go, Haskell, Java, JavaScript, Kotlin, Markdown, Python, Ruby, Rust, Swift, TypeScript/TSX

`zat` exits with code 1 for unsupported languages.
````
