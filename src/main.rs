mod pkg_parser;

use std::path::Path;

use crate::pkg_parser::parser::Pkg;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 1 {
        return;
    }

    let mut pkg = Pkg::new(Path::new(&args[0]));
    pkg.save_pkg(Path::new("./output"));
}
