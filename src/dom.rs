use std::collections::{HashMap, VecDeque};

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

struct NodeQueue {
    node: Node,
    level: u32
}

pub fn print(root_node: Node) {
    let mut node_q: VecDeque<NodeQueue> = VecDeque::new();

    node_q.push_back(NodeQueue { node: root_node, level: 0 });

    while !node_q.is_empty() {
        // Print Node content with tree level as indentation
        let current = node_q.pop_front().unwrap();
        for _ in 0..current.level {
            print!(" ");
        }
        match current.node.node_type {
            NodeType::Element(data) => {
                print!("{}: {:?}\n", data.tag_name, data.attributes)
            }
            NodeType::Text(txt) => {
                print!("{}\n", txt)
            }
        }

        // Add the children to the stack to traverse the tree
        let mut rev_child: Vec<Node> = Vec::new();
        for child in current.node.children {
          rev_child.push(child);
        }
        rev_child.reverse();

        for child in rev_child {
            node_q.push_front(NodeQueue {
                node: child,
                level: current.level + 1 });
        }
    }
}
