mod pkg_parser;

use crate::pkg_parser::parser::Pkg;

fn main() {
    const PATH: &str = "./test/scene.pkg";
    let mut pkg = Pkg::new(PATH);
}
