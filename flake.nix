{
  description = "cat for LLMs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    lat-js-viewer.url = "github:bglgwyng/lat-js-viewer";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          defaultFallback = pkgs.writeShellScriptBin "lat-fallback" ''
            file="$1"
            total=$(wc -l < "$file")
            limit=20
            if [ "$total" -le "$limit" ]; then
              cat -n "$file"
            else
              head -n "$limit" "$file" | cat -n
              echo "... ($total lines total)"
            fi
          '';
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
            config = {
              allowBroken = true;
            };
          };
          packages.default = pkgs.lib.makeOverridable (
            {
              rules ? [
                {
                  patterns = [
                    "*.js"
                    "*.ts"
                    "*.jsx"
                    "*.tsx"
                    "*.cjs"
                    "*.mjs"
                  ];
                  handler = inputs'.lat-js-viewer.packages.default;
                }
              ],
              fallback ? defaultFallback,
            }:
            let
              mkCase = rule: ''
                ${builtins.concatStringsSep "|" rule.patterns})
                  exec ${rule.handler}/bin/* "$file" "$@"
                  ;;
              '';
              cases = builtins.concatStringsSep "\n" (map mkCase rules);
            in
            (pkgs.writeShellScriptBin "lat" ''
              file="$1"
              shift
              case "$file" in
                ${cases}
                *)
                  exec ${fallback}/bin/* "$file" "$@"
                  ;;
              esac
            '').overrideAttrs
              {
                pname = "lat";
                version = "0.1.0";
              }
          ) { };
          packages.tarball = pkgs.runCommand "lat-${system}.tar.gz" { } ''
            tar -czvf $out -C ${self'.packages.default}/bin .
          '';
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
