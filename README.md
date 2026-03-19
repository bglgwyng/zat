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
| Python | [zat-python-viewer](https://github.com/bglgwyng/zat-python-viewer) | regex |
| Other | built-in fallback | head -n 20 |

## Nix Customization

Viewers, entry files, and fallback are all configurable via Nix override:

```nix
(zat.packages.default.override {
  directoryIndex = [ "index.ts" "mod.rs" "__init__.py" ];
  # Add or replace rules/fallback as needed
})
```

## For AI Agents

Add this to your `CLAUDE.md` or `AGENTS.md`:

```
Use `zat <file>` to see file outlines (exported symbols + line numbers).
Use `zat <dir>` to see directory outlines.
Then use Read with offset/limit to read specific line ranges.
```
