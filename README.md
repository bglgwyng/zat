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
pub struct OutlineEntry // L8-L12
pub struct ImportEntry // L14-L18
pub fn extract_outline(source: &str) -> OutlineResult // L25-L166
```

Only public/exported symbols are shown. Imports referenced in signatures are included.

### Directory outline

```shell
zat src/
```

```
lib.rs:
  pub struct Config // L5-L10
  pub fn load(path: &str) -> Config // L12-L30
main.rs:
```

Looks for entry files (`index.ts`, `lib.rs`, `__init__.py`, etc.) and shows their outlines. Falls back to file listing if no entry files found.

## Supported Languages

| Language | Viewer | Parser |
|----------|--------|--------|
| JS/TS/JSX/TSX | [zat-js-viewer](https://github.com/bglgwyng/zat-js-viewer) | oxc |
| Rust | [zat-rust-viewer](https://github.com/bglgwyng/zat-rust-viewer) | syn |
| Python | [zat-python-viewer](https://github.com/bglgwyng/zat-python-viewer) | rustpython-parser |
| Other | built-in fallback | cat -n |

## Nix Customization

### Override

Viewers, entry files, and fallback are all configurable via Nix override:

```nix
(zat.packages.default.override {
  directoryIndex = [ "index.ts" "mod.rs" "__init__.py" ];
  # Add or replace rules/fallback as needed
})
```

### NixOS Module

```nix
# flake.nix inputs
inputs.zat.url = "github:bglgwyng/zat";

# configuration.nix
{ inputs, ... }:
{
  imports = [ inputs.zat.nixosModules.default ];

  programs.zat = {
    enable = true;
    rules = [
      {
        patterns = [ "*.js" "*.ts" "*.tsx" ];
        handler = inputs.zat-js-viewer.packages.${system}.default;
      }
    ];
    # "." in directoryIndex controls where `ls` appears in output
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
Use it to understand file structure before reading the full content.

- `zat <file>` — Show outline (JS/TS, Rust, Python supported; falls back to `cat -n` for other types)
- `zat <dir>` — Show entry file outlines (index.ts, lib.rs, etc.) and directory listing
- Use the line numbers (e.g. `L10-L25`) to `Read(offset, limit)` into specific ranges
- Pipe through `head` to limit output: `zat <file> | head -n 30`
````
