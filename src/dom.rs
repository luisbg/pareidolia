use std::collections::HashMap;

pub type StrMap = HashMap<String, String>;

pub struct Node {
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

pub enum NodeType {
    Element(ElementData),
    Text(String),
}

pub struct ElementData {
    pub tag_name: String,
    pub attributes: StrMap,
}

pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: StrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
