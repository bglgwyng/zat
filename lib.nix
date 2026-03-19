{ pkgs }:

{
  defaultFallback = pkgs.writeShellScriptBin "zat-fallback" ''
    cat -n "$1"
  '';

  mkZat =
    { rules, fallback, directoryIndex }:
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

      # Directory support
      if [ -d "$file" ]; then
        printed=0
        entry_files=(${builtins.concatStringsSep " " (map (e: ''"${e}"'') directoryIndex)})
        for entry in "''${entry_files[@]}"; do
          if [ "$entry" = "." ]; then
            [ "$printed" -eq 1 ] && echo ""
            ${pkgs.coreutils}/bin/ls -1 "$file"
            printed=1
          else
            target="$file/$entry"
            if [ -f "$target" ]; then
              [ "$printed" -eq 1 ] && echo ""
              echo "$entry:"
              "$0" "$target" "$@" | ${pkgs.gnused}/bin/sed 's/^/  /'
              printed=1
            fi
          fi
        done
        exit 0
      fi

      case "$file" in
        ${cases}
        *)
          exec ${fallback}/bin/* "$file" "$@"
          ;;
      esac
    '').overrideAttrs {
      pname = "zat";
      version = "0.1.0";
    };
}
