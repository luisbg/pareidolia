// box model. All sizes are in px.

use style::{StyledNode, Display};
use css::Value::{Keyword, Length};
use css::Unit::Px;
use std::collections::VecDeque;
use dom::NodeType;

pub use self::BoxType::{AnonymousBlock, BlockNode};

#[derive(Default, Clone, Copy)]
pub struct Dimensions {
    // Position of the content area relative to the document origin:
    pub content: Rect,

    // Surrounding edges:
    padding: EdgeSizes,
    border: EdgeSizes,
}

#[derive(Default, Clone, Copy)]
pub struct Rect {
    x: f32,
    y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Clone, Copy)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

/// A node in the layout tree.
#[derive(Clone)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Clone)]
pub enum BoxType<'a> {
    BlockNode(&'a StyledNode<'a>),
    AnonymousBlock,
}

/// Transform a style tree into a layout tree.
pub fn layout_tree<'a>(node: &'a StyledNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
    // The layout algorithm expects the container height to start at 0.
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

/// Build the tree of LayoutBoxes, but don't perform any layout calculations yet.
fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> LayoutBox<'a> {
    // Create the root box.
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BlockNode(style_node),
        Display::None => panic!("Root node has display: none.")
    });

    // Create the descendant boxes.
    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::None => {} // Don't lay out nodes with `display: none;`
        }
    }
    root
}

impl<'a> LayoutBox<'a> {
    /// Lay out a box and its descendants.
    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BlockNode(_) => self.layout_block(containing_block),
            AnonymousBlock => {}
        }
    }

    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    /// Lay out a block-level element and its descendants.
    fn layout_block(&mut self, containing_block: Dimensions) {
        // Child width can depend on parent width, so we need to calculate this box's width before
        // laying out its children.
        self.calculate_block_width();

        // Determine where the box is located within its container.
        self.calculate_block_position(containing_block);

        // Recursively lay out the children of this box.
        self.layout_block_children();

        // Parent height can depend on child height, so `calculate_height` must be called after the
        // children are laid out.
        self.calculate_block_height();
    }

    /// Calculate the width of a block-level non-replaced element in normal flow.
    ///
    /// http://www.w3.org/TR/CSS2/visudet.html#blockwidth
    ///
    /// Sets the horizontal padding/border dimensions, and the `width`.
    fn calculate_block_width(&mut self) {
        let style = self.get_style_node();

        // `width` has initial value `auto`.
        let auto = Keyword("auto".to_string());
        let width = style.value("width").unwrap_or(auto.clone());

        // border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();
    }

    /// Finish calculating the block's edge sizes, and position it within its containing block.
    ///
    /// http://www.w3.org/TR/CSS2/visudet.html#normal-block
    ///
    /// Sets the vertical padding/border dimensions, and the `x`, `y` values.
    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        // border, and padding have initial value 0.
        let zero = Length(0.0, Px);

        d.border.top = style.lookup("border-top-width", "border-width", &zero).to_px();
        d.border.bottom = style.lookup("border-bottom-width", "border-width", &zero).to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.border.left + d.padding.left;

        // Position the box below all the previous boxes in the container.
        d.content.y = containing_block.content.height + containing_block.content.y +
                      d.border.top + d.padding.top;
    }

    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BlockNode(node) => node,
            AnonymousBlock => panic!("Anonymous block box has no style node")
        }
    }

    /// Lay out the block's children within its content area.
    ///
    /// Sets `self.dimensions.height` to the total content height.
    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Increment the height so each child is laid out below the previous one.
            d.content.height = d.content.height + child.dimensions.content.height;
        }
    }

    /// Height of a block-level non-replaced element in normal flow with overflow visible.
    fn calculate_block_height(&mut self) {
        // If the height is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by `layout_block_children`.
        if let Some(Length(h, Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}

struct NodeQueue<'a> {
    lbnode: LayoutBox<'a>,
    level: u32
}

pub fn print(root_node: LayoutBox) {
    println!("Layout tree:");
    let mut node_q: VecDeque<NodeQueue> = VecDeque::new();

    node_q.push_back(NodeQueue { lbnode: root_node, level: 0 });

    while !node_q.is_empty() {
        // Print Node content with tree level as indentation
        let current = node_q.pop_front().unwrap();

        for _ in 0..current.level {
            print!(" ");
        }

        // print node info
        match current.lbnode.box_type {
            BlockNode(sn) => {
                let name = match sn.node.node_type {
                    NodeType::Element(ref e) => ("elem", &e.tag_name),
                    NodeType::Text(ref s) => ("txt", s),
                };
                print!("{}: {} -- ({},{}) [{},{}]", name.0, name.1,
                       current.lbnode.dimensions.content.x,
                       current.lbnode.dimensions.content.y,
                       current.lbnode.dimensions.content.width,
                       current.lbnode.dimensions.content.height);
            }
            _ => ()
        }

        // Add the children to the stack to traverse the tree
        let mut rev_child: Vec<LayoutBox> = Vec::new();
        for child in current.lbnode.children {
          rev_child.push(child);
        }
        rev_child.reverse();

        for child in rev_child {
            node_q.push_front(NodeQueue {
                lbnode: child,
                level: current.level + 1 });
        }

        print!("\n");
    }

    println!("");
}
