mod pkg_parser;

use std::path::Path;

use crate::pkg_parser::parser::Pkg;

fn main() {
    const PATH: &str = "./test/scene.pkg";
    let mut pkg = Pkg::new(Path::new(PATH));
    pkg.save_pkg(Path::new("./output"));
}
