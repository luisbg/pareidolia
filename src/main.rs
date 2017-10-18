extern crate getopts;
extern crate image;

pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod painting;

use std::fs::File;
use std::io::{Read, BufWriter};

fn main() {
    println!("Lanch Pareidolia\n");

    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    // Since we don't have an actual window, hard-code the "viewport" size.
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width  = 800.0;
    viewport.content.height = 600.0;

    // Read input file
    let html = read_source(str_arg("h", "examples/test.html"));

    // Parsing
    let root_node = html::parse(html);
    let stylesheet = css::example();
    let styled = style::style_tree(&root_node, &stylesheet);
    let layout = layout::layout_tree(&styled, viewport);
    let display_list = painting::build_display_list(&layout);

    // Print for simple visualization
    dom::print(root_node.clone());
    style::print(styled.clone());
    layout::print(layout.clone());
    painting::print(display_list.clone());

    // Save an image:
    let filename = String::from("output.png");
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    let canvas = painting::Canvas::paint(&display_list, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
        let color = canvas.pixels[(y * w + x) as usize];
        image::Pixel::from_channels(color.r, color.g, color.b, color.a)
    });
    image::ImageRgba8(img).save(&mut file, image::PNG).is_ok();
    println!("Saved output as {}", filename);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}
