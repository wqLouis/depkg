mod pkg_parser;

use crate::pkg_parser::parser::Pkg;
use clap::{ArgAction, Parser};
use std::path::Path;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    pkg_path: String,

    #[arg(short, long, default_value = ".")]
    target_path: String,

    /// Dry run without saving the file [default: false]
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Parse texture into image [default: true]
    #[arg(long, default_value_t = true, action = ArgAction::SetFalse)]
    parse_tex: bool,

    /// Verbose output [defualt: false]
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let mut pkg = Pkg::new(Path::new(&args.pkg_path));
    pkg.save_pkg(
        Path::new(&args.target_path),
        args.dry_run,
        args.parse_tex,
        args.verbose,
    );
}
