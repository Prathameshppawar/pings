//! The `pings` command-line tool.
//!
//! Phase 0: only `--help` and `--version`. The REPL and `pings run <file>`
//! arrive in Phase 1 (see kb/roadmap.md).

use std::env;
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP: &str = "\
pings: the Pings programming language (pre-alpha)

USAGE:
    pings [OPTIONS]

OPTIONS:
    -h, --help       Print this help
    -V, --version    Print version

Pings can't run code yet; the lexer is being built. Follow along:
https://github.com/Prathameshppawar/pings/tree/main/kb/journal
";

fn main() -> ExitCode {
    // std::env::args is enough for two flags. We'll pick an argument parser
    // (or write one) when `pings run` and the REPL need subcommands.
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        None | Some("-h" | "--help") => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Some("-V" | "--version") => {
            println!("pings {VERSION}");
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("pings: unknown argument '{other}'\n");
            eprint!("{HELP}");
            // 2 = usage error, the common Unix convention.
            ExitCode::from(2)
        }
    }
}
