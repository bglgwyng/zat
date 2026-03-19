{
  description = "zat: cat for LLMs";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    zat-js-viewer = {
      url = "github:bglgwyng/zat-js-viewer";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zat-rust-viewer = {
      url = "github:bglgwyng/zat-rust-viewer";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    zat-python-viewer = {
      url = "github:bglgwyng/zat-python-viewer";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];

      flake = {
        nixosModules.default = ./module.nix;
        darwinModules.default = ./module.nix;
      };

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
          zatLib = import ./lib.nix { inherit pkgs; };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            config = {
              allowBroken = true;
            };
          };
          packages.default = zatLib.mkZat {
            rules = [
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
            ];
            fallback = zatLib.defaultFallback;
            directoryIndex = [
              "index.md"
              "README.md"
              "index.ts"
              "index.js"
              "index.tsx"
              "index.jsx"
              "mod.rs"
              "lib.rs"
              "main.rs"
              "__init__.py"
              "."
            ];
          };
          packages.tarball = pkgs.runCommand "zat-${system}.tar.gz" { } ''
            tar -czvf $out -C ${self'.packages.default}/bin .
          '';
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
