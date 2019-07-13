pub struct Packer {
    root: Box<RectNode>,
}

impl Packer {
    pub fn new(size: (u32, u32)) -> Self {
        Packer {
            root: Box::new(RectNode::Leaf(Rectangle::new(
                0,
                0,
                size.0 as i32,
                size.1 as i32,
            ))),
        }
    }
}

pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }
}

enum RectNode {
    Branch {
        rect: Rectangle,
        left: Box<RectNode>,
        right: Box<RectNode>,
    },

    Leaf(Rectangle),
}
