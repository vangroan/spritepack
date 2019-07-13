//! Split pattern
//!
//! ```ignore
//!  ____________________________
//! |          |                 |
//! |   Slot   |      Right      |
//! |          |                 |
//! |__________|_________________|
//! |                            |
//! |                            |
//! |            Left            |
//! |                            |
//! |                            |
//! |____________________________|
//! ```
//!

use crate::errors::{ErrorKind, Result};
use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, RgbaImage};
use std::fmt;

pub struct Packer {
    size: (u32, u32),
    root: Box<RectNode>,
}

impl Packer {
    pub fn new(size: (u32, u32)) -> Self {
        Packer {
            size,
            root: Box::new(RectNode::Leaf(Rectangle::new(0, 0, size.0, size.1))),
        }
    }

    pub fn pack<V>(&mut self, images: V) -> Result<RgbaImage>
    where
        V: Into<Vec<DynamicImage>>,
    {
        let mut images = images.into();

        // Sort heighest to shortest
        images.sort_by(|a, b| b.dimensions().1.cmp(&a.dimensions().1));

        let mut target: RgbaImage = ImageBuffer::new(self.size.0, self.size.1);

        for img in images {
            if let Some(pos) = self.root.make_slot(&img.dimensions()) {
                imageops::replace(&mut target, &img.to_rgba(), pos.0, pos.1);
            } else {
                // TODO: Optionally create new target image for multiple outputs
                return Err(ErrorKind::PackerOutOfSpace(img.dimensions()).into());
            }
        }

        Ok(target)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    /// Indicates whether the given width and height
    /// will fit inside this rectangle.
    pub fn can_fit(&self, size: &(u32, u32)) -> bool {
        size.0 <= self.width && size.1 <= self.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.x, self.y, self.width, self.height
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
enum RectNode {
    /// A leaf node that has no space left.
    Closed,

    Branch {
        rect: Rectangle,
        left: Box<RectNode>,
        right: Box<RectNode>,
    },

    Leaf(Rectangle),
}

impl RectNode {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            RectNode::Closed
        } else {
            RectNode::Leaf(Rectangle {
                x,
                y,
                width,
                height,
            })
        }
    }

    fn make_slot(&mut self, target: &(u32, u32)) -> Option<(u32, u32)> {
        use RectNode::*;

        match self {
            Closed => None,
            Branch { rect, left, right } => {
                if !rect.can_fit(&target) {
                    // Don't bother recursing deeper if
                    // the target is too large.
                    return None;
                }

                // Right side has precedence.
                let right_pos = right.make_slot(target);

                if right_pos.is_some() {
                    return right_pos;
                } else {
                    return left.make_slot(target);
                }
            }
            Leaf(rect) => {
                if rect.can_fit(&target) {
                    let slot = Some((rect.x, rect.y));

                    // Split this node
                    *self = Branch {
                        rect: rect.clone(),
                        right: Box::new(RectNode::new(
                            rect.x + target.0,
                            rect.y,
                            rect.width - target.0,
                            target.1,
                        )),
                        left: Box::new(RectNode::new(
                            rect.x,
                            rect.y + target.1,
                            rect.width,
                            rect.height - target.1,
                        )),
                    };

                    slot
                } else {
                    None
                }
            }
        }
    }

    /// Right node of branch.
    ///
    /// `None` when node is not a branch.
    fn right(&self) -> Option<&RectNode> {
        use RectNode::*;

        match self {
            Branch { right, .. } => Some(right),
            _ => None,
        }
    }

    /// Left node of branch.
    ///
    /// `None` when node is not a branch.
    fn left(&self) -> Option<&RectNode> {
        use RectNode::*;

        match self {
            Branch { left, .. } => Some(left),
            _ => None,
        }
    }

    /// Rectangle of branch and leaf nodes.
    fn rect(&self) -> Option<&Rectangle> {
        use RectNode::*;

        match self {
            Closed => None,
            Branch { rect, .. } => Some(rect),
            Leaf(rect) => Some(rect),
        }
    }

    /// Prints the string representation of the
    /// node tree into the given writer, recursively.
    fn repr(&self, f: &mut fmt::Formatter<'_>, depth: u32) -> fmt::Result {
        use RectNode::*;

        let padding: String = (0..depth).map(|_| "  ").collect();

        match self {
            Closed => writeln!(f, "{} * Closed", padding),
            Branch { rect, right, left } => {
                writeln!(f, "{} - {}", padding, rect)?;
                right.repr(f, depth + 1)?;
                left.repr(f, depth + 1)?;

                Ok(())
            }
            Leaf(rect) => writeln!(f, "{} * {}", padding, rect),
        }
    }
}

impl fmt::Display for RectNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.repr(f, 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let mut root = RectNode::Leaf(Rectangle::new(0, 0, 10, 10));

        let slot_1 = root.make_slot(&(5, 5));

        assert_eq!(slot_1, Some((0, 0)));
        assert_eq!(
            root.right().and_then(RectNode::rect),
            Some(&Rectangle::new(5, 0, 5, 5))
        );
        assert_eq!(
            root.left().and_then(RectNode::rect),
            Some(&Rectangle::new(0, 5, 10, 5))
        );

        let slot_2 = root.make_slot(&(4, 5));

        assert_eq!(
            slot_2,
            Some((5, 0)),
            "Slot was not placed in the right side node"
        );
        assert_eq!(
            root.right()
                .and_then(RectNode::right)
                .and_then(RectNode::rect),
            Some(&Rectangle::new(9, 0, 1, 5))
        );
        assert_eq!(
            root.right().and_then(RectNode::left),
            Some(&RectNode::Closed)
        );
    }
}
