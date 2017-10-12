pub mod dom;

use dom::NodeType;

fn main() {
    println!("Lanch Petrichor\n");

    let hello = dom::text(String::from("Hello world"));

    match hello.node_type {
            NodeType::Element(_) => (),
            NodeType::Text(txt) => {
                println!("{}", txt)
            }
        }
}
