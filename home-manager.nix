{ zat }:
{
  lib,
  config,
  ...
}:
{
  options.programs.zat = {
    enable = lib.mkEnableOption "zat code outline viewer";
    claude-code = {
      enable = lib.mkEnableOption "claude-code integration for zat";
    };
  };

  config = lib.mkIf config.programs.zat.enable (
    lib.mkMerge [
      {
        home.packages = [ zat ];
      }
      (lib.mkIf config.programs.zat.claude-code.enable {
        programs.claude-code.rules.zat = ''
          ### zat

          A code outline viewer that shows exported symbol signatures with line numbers.

          Prefer `zat` over `cat`/`Read` when you need signatures, not full implementation. Use the line numbers in the output to `Read(offset, limit)` into specific sections.

          Supported languages: C, C++, C#, Go, Haskell, Java, JavaScript, Kotlin, Markdown, Python, Ruby, Rust, Swift, TypeScript/TSX

          `zat` exits with code 1 for unsupported languages.
        '';
      })
    ]
  );
}
