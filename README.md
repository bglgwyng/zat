# zat

`zat` is a code outline viewer. It shows exported symbols and line numbers at a glance.

Works great as a tool for LLM coding agents — they can see the outline first, then `Read` only the parts they need.

The name comes from Japanese ざっと (*zatto*), meaning "roughly" or "at a glance".

## Installation

### Nix

```shell
nix profile install github:bglgwyng/zat
```

Or run directly:

```shell
nix run github:bglgwyng/zat -- <FILE>
```

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

Only public/exported symbols are shown. Visibility modifiers and other boilerplate may be omitted for brevity. Struct fields and interface members are included.

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
main.rs:
```

Looks for entry files (`index.ts`, `lib.rs`, `__init__.py`, etc.) and shows their outlines. `"."` in `directoryIndex` controls where `ls` output appears.

## Supported Languages

| Language | Viewer | Parser |
|----------|--------|--------|
| JS/TS/JSX/TSX | [zat-js-viewer](https://github.com/bglgwyng/zat-js-viewer) | oxc |
| Rust | [zat-rust-viewer](https://github.com/bglgwyng/zat-rust-viewer) | syn |
| Python | [zat-python-viewer](https://github.com/bglgwyng/zat-python-viewer) | rustpython-parser |
| Other | built-in fallback | `cat -n` |

## Nix Customization

### NixOS / nix-darwin Module

```nix
# flake.nix inputs
inputs.zat.url = "github:bglgwyng/zat";

# configuration.nix
{ inputs, ... }:
{
  imports = [ inputs.zat.nixosModules.default ]; # or darwinModules.default

  programs.zat = {
    enable = true;
    rules = [
      {
        patterns = [ "*.ts" "*.tsx" ];
        handler = "${inputs.zat-js-viewer.packages.${system}.default}/bin/zat-js-viewer --lang ts";
      }
    ];
    directoryIndex = [ "index.ts" "lib.rs" "." ];
  };
}
```

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

- `zat <file>` — Show outline (JS/TS, Rust, Python supported; falls back to `cat -n` for other types)
- `zat <dir>` — Show entry file outlines (index.ts, lib.rs, etc.) and directory listing
- Use the line numbers (e.g. `L10-L25`) to `Read(offset, limit)` into specific ranges
- Pipe through `head` to limit output: `zat <file> | head -n 50`
````
