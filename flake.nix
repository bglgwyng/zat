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

      flake =
        let
          systemModule = import ./system-module.nix { inherit inputs; };
        in
        {
          nixosModules.default = systemModule;
          darwinModules.default = systemModule;
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
          lib,
          system,
          ...
        }:
        let
          zatSubmodule = import ./submodule.nix { inherit inputs; } { inherit pkgs lib; };
          evalZat = (lib.evalModules {
            modules = [
              zatSubmodule
              { enable = true; }
            ];
          }).config;
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            config = {
              allowBroken = true;
            };
          };
          packages.default = evalZat.package;
          packages.tarball = pkgs.runCommand "zat-${system}.tar.gz" { } ''
            tar -czvf $out -C ${self'.packages.default}/bin .
          '';
          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
