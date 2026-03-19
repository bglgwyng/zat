{
  description = "zat: cat for LLMs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    zat-js-viewer.url = "github:bglgwyng/zat-js-viewer";
    zat-rust-viewer.url = "github:bglgwyng/zat-rust-viewer";
    zat-python-viewer.url = "github:bglgwyng/zat-python-viewer";
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
          defaultFallback = pkgs.writeShellScriptBin "zat-fallback" ''
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
                  handler = inputs'.zat-js-viewer.packages.default;
                }
                {
                  patterns = [ "*.rs" ];
                  handler = inputs'.zat-rust-viewer.packages.default;
                }
                {
                  patterns = [ "*.py" ];
                  handler = inputs'.zat-python-viewer.packages.default;
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
            (pkgs.writeShellScriptBin "zat" ''
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
                pname = "zat";
                version = "0.1.0";
              }
          ) { };
          packages.tarball = pkgs.runCommand "zat-${system}.tar.gz" { } ''
            tar -czvf $out -C ${self'.packages.default}/bin .
          '';
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
