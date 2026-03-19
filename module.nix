{ pkgs, lib, config, ... }:

let
  cfg = config.programs.zat;
  zatLib = import ./lib.nix { inherit pkgs; };

  ruleType = lib.types.submodule {
    options = {
      patterns = lib.mkOption {
        type = lib.types.listOf lib.types.str;
        description = "File glob patterns to match (e.g. \"*.js\", \"*.ts\").";
      };
      handler = lib.mkOption {
        type = lib.types.package;
        description = "Viewer package to handle matched files.";
      };
    };
  };
in
{
  options.programs.zat = {
    enable = lib.mkEnableOption "zat, a code outline viewer";

    rules = lib.mkOption {
      type = lib.types.listOf ruleType;
      default = [ ];
      description = "List of file pattern rules mapping to viewer handlers.";
    };

    fallback = lib.mkOption {
      type = lib.types.package;
      default = zatLib.defaultFallback;
      defaultText = lib.literalExpression "built-in head -n 20 fallback";
      description = "Fallback viewer for unmatched file types.";
    };

    directoryIndex = lib.mkOption {
      type = lib.types.listOf lib.types.str;
      default = [
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
      description = "Entry filenames to look for when viewing a directory.";
    };

    package = lib.mkOption {
      type = lib.types.package;
      readOnly = true;
      description = "The built zat package (derived from configuration).";
    };
  };

  config = lib.mkIf cfg.enable {
    programs.zat.package = zatLib.mkZat {
      inherit (cfg) rules fallback directoryIndex;
    };

    environment.systemPackages = [ cfg.package ];
  };
}
