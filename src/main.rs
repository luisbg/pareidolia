extern crate getopts;

pub mod dom;
pub mod html;
pub mod css;
pub mod style;

use std::fs::File;
use std::io::Read;

fn main() {
    println!("Lanch Petrichor\n");

    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    // Read input file
    let html = read_source(str_arg("h", "examples/test.html"));

    // Parsing
    let root_node = html::parse(html);
    let stylesheet = css::example();
    let styled = style::style_tree(&root_node, &stylesheet);

    // Print for simple visualization
    dom::print(root_node.clone());
    style::print(styled.clone());
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
