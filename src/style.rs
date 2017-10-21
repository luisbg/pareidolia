use dom::{Node, NodeType, ElementData};
use css::{Selector, Rule, Stylesheet, Value, SimpleSelector, Specificity};
use std::collections::{HashMap, VecDeque};

// Map from CSS property names to values.
type PropertyMap = HashMap<String, Value>;

// A node with associated style data.
#[derive(Clone)]
pub struct StyledNode<'a> {
    pub node: &'a Node, // pointer to a DOM node
    specified_values: PropertyMap,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Horizontal,
    Vertical,
    None
}

impl<'a> StyledNode<'a> {
    /// Return the specified value of a property if it exists, otherwise `None`.
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).cloned()
    }

    /// The value of the `display` property (defaults to vertical).
    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match &*s {
                "none" => Display::None,
                "horizontal" => {
                    println!("LBG: Got something horizontal");
                    Display::Horizontal
                },
                _ => Display::Vertical
            },
            _ => Display::Vertical
        }
    }

    /// Return the specified value of property `name`, or property `fallback_name` if that doesn't
    /// exist. or value `default` if neither does.
    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name).unwrap_or_else(|| self.value(fallback_name)
                        .unwrap_or_else(|| default.clone()))
    }
}

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref simple_selector) => matches_simple_selector(elem, simple_selector)
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let elem_classes = elem.classes();
    if selector.class.iter().any(|class| !elem_classes.contains(&**class)) {
        return false;
    }

    // We didn't find any non-matching selector components.
    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// If `rule` matches `elem`, return a `MatchedRule`. Otherwise return `None`.
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    // Find the first (highest-specificity) matching selector.
    rule.selectors.iter()
        .find(|selector| matches(elem, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element.
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

// Apply styles to a single element, returning the specified values.
fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(elem, stylesheet);

    // Go through the rules from lowest to highest specificity.
    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }
    return values;
}

// Apply a stylesheet to an entire DOM tree, returning a StyledNode tree.
pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            NodeType::Text(_) => HashMap::new()
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

struct NodeQueue<'a> {
    stnode: StyledNode<'a>,
    level: u32
}

pub fn print(root_node: StyledNode) {
    println!("Style tree:");
    let mut node_q: VecDeque<NodeQueue> = VecDeque::new();

    node_q.push_back(NodeQueue { stnode: root_node, level: 0 });

    while !node_q.is_empty() {
        // Print Node content with tree level as indentation
        let current = node_q.pop_front().unwrap();

        for _ in 0..current.level {
            print!(" ");
        }

        match current.stnode.node.node_type {
            NodeType::Element(ref e) => { print!("elem: {} ", e.tag_name) },
            NodeType::Text(ref t) => { print!("txt: {} ", t) }
        }

        for (s, v) in current.stnode.specified_values.iter() {
            print!(".{}=", s);
            match *v {
                Value::Keyword(_) => ( print!("? ")),
                Value::Length(l, _) => (print!("{}px ", l)),
                Value::ColorValue(ref c) => {
                    print!("{}r-{}g-{}b ", c.r, c.g, c.b)
                }
            }
        }

        // Add the children to the stack to traverse the tree
        let mut rev_child: Vec<StyledNode> = Vec::new();
        for child in current.stnode.children {
          rev_child.push(child);
        }
        rev_child.reverse();

        for child in rev_child {
            node_q.push_front(NodeQueue {
                stnode: child,
                level: current.level + 1 });
        }

        print!("\n");
    }

    println!("");
}
