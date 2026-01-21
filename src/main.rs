mod pkg_parser;

use std::path::Path;

use glob::{Paths, glob};

use crate::pkg_parser::parser::Pkg;

fn main() {
    const PATH: &str = "./test/scene.pkg";

    for i in load_scene() {
        let path = i.unwrap();
        let mut pkg = Pkg::new(&path);
        pkg.save_pkg(Path::new("./output"));
    }

    //let mut pkg = Pkg::new(Path::new(PATH));
    //pkg.save_pkg(Path::new("./output"));
}

fn load_scene() -> Paths {
    const WORKSHOP: &str = "/home/wqlouis/.steam/steam/steamapps/workshop/content/431960";
    glob(&(WORKSHOP.to_owned() + "/**/*.pkg")).unwrap()
}
