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
    flake-parts.lib.mkFlake { inherit inputs; } {
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
            version = "0.3.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              (pkgs.rust-bin.stable."1.85.0".default.override {
                extensions = [ "rust-src" ];
              })
            ];
          };
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
