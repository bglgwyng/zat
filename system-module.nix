{ inputs }:
{ lib, config, ... }:

{
  imports = [ (import ./module.nix { inherit inputs; }) ];

  config = lib.mkIf config.programs.zat.enable {
    environment.systemPackages = [ config.programs.zat.package ];
  };
}
