{ zat }:
{
  lib,
  config,
  ...
}:
let
  zatPrompt = ''
    ### zat

    A code outline viewer that shows exported symbol signatures with line numbers.

    Prefer `zat` over `cat` when you need signatures, not full implementation. Use the line numbers in the output to inspect specific sections.

    Supported languages: C, C++, C#, Go, Haskell, Java, JavaScript, Kotlin, Markdown, Python, Ruby, Rust, Swift, TypeScript/TSX

    `zat` exits with code 1 for unsupported languages.
  '';
in
{
  options.programs.zat = {
    enable = lib.mkEnableOption "zat code outline viewer";
    claude-code = {
      enable = lib.mkEnableOption "claude-code integration for zat";
    };
    codex = {
      enable = lib.mkEnableOption "Codex integration for zat";
    };
  };

  config = lib.mkIf config.programs.zat.enable (
    lib.mkMerge [
      {
        home.packages = [ zat ];
      }
      (lib.mkIf config.programs.zat.claude-code.enable {
        programs.claude-code.rules.zat = zatPrompt;
      })
      (lib.mkIf config.programs.zat.codex.enable {
        programs.codex.context = zatPrompt;
      })
    ]
  );
}
