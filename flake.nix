{
  description = "zat - code outline viewer";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } (
      { withSystem, ... }:
      {
        imports = [
          inputs.flake-parts.flakeModules.easyOverlay
        ];
        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "aarch64-darwin"
          "x86_64-darwin"
        ];
        perSystem =
          {
            pkgs,
            system,
            config,
            ...
          }:
          {
            _module.args.pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                (import inputs.rust-overlay)
              ];
              config = { };
            };
            packages.default = pkgs.rustPlatform.buildRustPackage {
              pname = "zat";
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
            devShells.default = pkgs.mkShell {
              nativeBuildInputs = [
                (pkgs.rust-bin.stable."1.95.0".default.override {
                  extensions = [ "rust-src" ];
                })
              ];
            };
            overlayAttrs = {
              zat = config.packages.default;
            };
            formatter = pkgs.nixfmt-rfc-style;
          };
        flake.homeManagerModules.default =
          { pkgs, ... }:
          {
            imports = [
              (import ./home-manager.nix {
                zat = withSystem pkgs.stdenv.hostPlatform.system ({ config, ... }: config.packages.default);
              })
            ];
          };
      }
    );
}
