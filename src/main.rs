mod outline;

use std::fs;
use std::path::Path;

use outline::write_outline;
use zat::lang_for_ext;
use zat::lang_for_name;
use zat::supported_languages;

/// Parsed command-line arguments.
struct Options {
    file: Option<String>,
    lang: Option<String>,
}

/// Parse `args` (the raw arguments, excluding argv[0]).
///
/// Returns `Ok(Options)` on a well-formed command line, or `None` if the
/// process should exit immediately (either after printing `--help`/`--version`
/// with a success status, or after reporting a usage error).
fn parse_args(args: &[String]) -> Option<Options> {
    let mut file: Option<String> = None;
    let mut lang: Option<String> = None;
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("zat {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            "--" => {
                // Everything after `--` is a positional argument.
                for rest in &args[i + 1..] {
                    if file.is_none() {
                        file = Some(rest.clone());
                    }
                }
                break;
            }
            "-l" | "--lang" => {
                let value = args.get(i + 1).cloned();
                match value {
                    Some(v) => {
                        lang = Some(v);
                        i += 1;
                    }
                    None => {
                        eprintln!("zat: option '{arg}' requires a value");
                        eprintln!("Try 'zat --help' for more information.");
                        std::process::exit(2);
                    }
                }
            }
            _ if arg.starts_with("--lang=") => {
                lang = Some(arg["--lang=".len()..].to_string());
            }
            _ if arg.starts_with('-') => {
                eprintln!("zat: unknown option '{arg}'");
                eprintln!("Try 'zat --help' for more information.");
                std::process::exit(2);
            }
            _ => {
                if file.is_none() {
                    file = Some(arg.clone());
                }
            }
        }

        i += 1;
    }

    Some(Options { file, lang })
}

fn print_help() {
    println!(
        "zat {} - a code outline viewer\n\
\n\
        USAGE:\n\
        \x20   zat [OPTIONS] <FILE>\n\
\n\
        ARGS:\n\
        \x20   <FILE>    Path to a source file to outline\n\
\n\
        OPTIONS:\n\
        \x20   -l, --lang <LANG>    Parse FILE as this language instead of\n\
        \x20                       inferring it from the file extension.\n\
        \x20                       Accepts a language name or extension alias\n\
        \x20                       (e.g. 'rust', 'rs', 'python', 'py').\n\
        \x20   -h, --help           Print this help message and exit\n\
        \x20   -V, --version        Print version and exit\n\
\n\
        SUPPORTED LANGUAGES:\n\
        \x20   {}\n\
\n\
        EXIT CODES:\n\
        \x20   0   Success\n\
        \x20   1   File or path error (missing file, directory, unknown type)\n\
        \x20   2   Usage error (unknown option or language)",
        env!("CARGO_PKG_VERSION"),
        supported_languages().join(", "),
    );
}

fn print_outline(source: &str, ranges: &[outline::VisibleRange<'_>], prefix: &str) {
    let mut out = std::io::stdout().lock();
    let _ = write_outline(source, ranges, prefix, &mut out);
}

fn view_file(path: &Path, lang: Option<&str>) {
    let resolved = match lang {
        Some(name) => match lang_for_name(name) {
            Some(spec) => spec,
            None => {
                eprintln!("zat: unknown language: {name}");
                eprintln!("Supported languages: {}", supported_languages().join(", "));
                std::process::exit(2);
            }
        },
        None => {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            match lang_for_ext(ext) {
                Some(spec) => spec,
                None => {
                    eprintln!("zat: {}: unknown file type", path.display());
                    std::process::exit(1);
                }
            }
        }
    };

    let (language, query_src) = resolved;
    let source = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("zat: {}: {}", path.display(), e);
        std::process::exit(1);
    });
    let tree = match outline::parse(&source, &language) {
        Some(t) => t,
        None => return,
    };
    let ranges = outline::extract_outline(&source, &tree, &language, query_src);
    print_outline(&source, &ranges, "");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let options = match parse_args(&args) {
        Some(o) => o,
        None => std::process::exit(1),
    };

    let path_arg = options.file.unwrap_or_else(|| {
        eprintln!("Usage: zat [OPTIONS] <FILE>");
        eprintln!("Try 'zat --help' for more information.");
        std::process::exit(1);
    });

    let path = Path::new(&path_arg);
    if !path.exists() {
        eprintln!("zat: {}: No such file or directory", path.display());
        std::process::exit(1);
    }

    if path.is_dir() {
        eprintln!("zat: {}: is a directory", path.display());
        std::process::exit(1);
    } else {
        view_file(path, options.lang.as_deref());
    }
}