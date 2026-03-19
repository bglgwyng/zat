# Contributing

We currently have the following viewers implemented:

- **JSON**: [zat-json-viewer](https://github.com/bglgwyng/zat-json-viewer)
- **JS/TS**: [zat-js-viewer](https://github.com/bglgwyng/zat-js-viewer)
- **Plaintext (fallback)**: [zat-plaintext-viewer](https://github.com/bglgwyng/zat-plaintext-viewer)

## Adding Support for New File Types

To extend `zat` to support more file extensions, you can create your own viewer. We welcome pull requests for both new viewers and improvements to existing ones!

## Distribution Notes

For the Nix distribution, users will be able to add custom file extension support and replace existing viewers with their own implementations. Non-Nix distributions will ship with a fixed set of standard viewers and won't be configurable.
