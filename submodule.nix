{ inputs }:
{ pkgs, lib }:
let
  zatLib = import ./lib.nix { inherit pkgs; };

  viewers = {
    zat-js-viewer = inputs.zat-js-viewer.packages.${pkgs.stdenv.hostPlatform.system}.default;
    zat-rust-viewer = inputs.zat-rust-viewer.packages.${pkgs.stdenv.hostPlatform.system}.default;
    zat-python-viewer = inputs.zat-python-viewer.packages.${pkgs.stdenv.hostPlatform.system}.default;
  };

  defaultRules = [
    {
      patterns = [ "*.js" "*.jsx" "*.cjs" "*.mjs" ];
      handler = "${viewers.zat-js-viewer}/bin/zat-js-viewer --lang js";
    }
    {
      patterns = [ "*.ts" "*.tsx" ];
      handler = "${viewers.zat-js-viewer}/bin/zat-js-viewer --lang ts";
    }
    {
      patterns = [ "*.rs" ];
      handler = "${viewers.zat-rust-viewer}/bin/zat-rust-viewer";
    }
    {
      patterns = [ "*.py" ];
      handler = "${viewers.zat-python-viewer}/bin/zat-python-viewer";
    }
  ];

  ruleType = lib.types.submodule {
    options = {
      patterns = lib.mkOption {
        type = lib.types.listOf lib.types.str;
        description = "File glob patterns to match (e.g. \"*.js\", \"*.ts\").";
      };
      handler = lib.mkOption {
        type = lib.types.str;
        description = "Command to handle matched files (receives content via stdin).";
      };
    };
  };
in
{ config, ... }:
{
  options = {
    enable = lib.mkEnableOption "zat, a code outline viewer";

    rules = lib.mkOption {
      type = lib.types.listOf ruleType;
      default = defaultRules;
      description = "List of file pattern rules mapping to viewer handlers.";
    };

    fallback = lib.mkOption {
      type = lib.types.package;
      default = zatLib.defaultFallback;
      defaultText = lib.literalExpression "built-in cat -n fallback";
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

  config = lib.mkIf config.enable {
    package = zatLib.mkZat {
      inherit (config) rules fallback directoryIndex;
    };
  };
}
