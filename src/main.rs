mod pkg_parser;

use std::path::Path;

use crate::pkg_parser::parser::Pkg;

fn main() {
    const PATH: &str = "./test/scene.pkg";
    let pkg = Pkg::new(Path::new(PATH));

    for i in pkg.entries {
        println!("{}", i.path);
    }
}
