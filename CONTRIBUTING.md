# Contributing

## Current Viewers

- **JS/TS**: [zat-js-viewer](https://github.com/bglgwyng/zat-js-viewer) — oxc parser
- **Rust**: [zat-rust-viewer](https://github.com/bglgwyng/zat-rust-viewer) — syn parser
- **Python**: [zat-python-viewer](https://github.com/bglgwyng/zat-python-viewer) — regex-based

## Adding a New Viewer

1. Create a Rust CLI that takes a file path and outputs outline entries with line numbers
2. Output format: `signature // L{start}-L{end}` (one per line, multi-line for classes/traits)
3. Filter imports to only show those referenced in signatures
4. Add a `flake.nix` for Nix builds (see existing viewers for reference)
5. Add the viewer to `zat/flake.nix` as an input with file extension patterns

## Nix Customization

Users can override the default configuration:

```nix
zat.packages.default.override {
  rules = [ ... ];          # File extension → viewer mapping
  fallback = ...;           # Handler for unknown file types
  directoryIndex = [ ... ]; # Entry files to look for in directories
}
```
