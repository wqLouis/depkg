# Wallpaper Engine Pkg Parser

A Rust library and tool for parsing Wallpaper Engine `.pkg` files and `.tex` textures.

## Features
*   Extracts files from `.pkg` archives while maintaining directory structure.
*   Converts `.tex` texture files to `.png` (**WIP** - support is currently experimental).

## Disclaimer
**Please respect the copyright of wallpaper creators.** This tool is intended for bring wallpaper engine to Linux and educational purposes only. Do not use this tool to pirate or distribute paid content.

## Usage

```rust
use std::path::Path;
use pkg_parser::Pkg; // Replace with your actual crate name

fn main() {
    let pkg_path = Path::new("wallpaper.pkg");
    let pkg = Pkg::new(pkg_path);
    
    // Extracts all files to the "output" directory
    pkg.save_pkg(Path::new("output"));
}
```