{ inputs }:
{ pkgs, lib, ... }:
let
  zatSubmodule = import ./submodule.nix { inherit inputs; } { inherit pkgs lib; };
in
{
  options.programs.zat = lib.mkOption {
    type = lib.types.submodule zatSubmodule;
    default = { };
    description = "zat code outline viewer configuration.";
  };
}
