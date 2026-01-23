mod pkg_parser;

use std::path::Path;

use crate::pkg_parser::parser::Pkg;

fn main() {
    let mut pkg = Pkg::new(Path::new("./test/scene.pkg"));
    pkg.save_pkg(Path::new("./output"));
}
