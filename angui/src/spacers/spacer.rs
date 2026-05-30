use crate::{
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
};

/// Spacer that takes up a specified vertical space.
/// Does not take up any space horizontally.
pub struct VerticalSpacer {
    height: usize,
}

impl VerticalSpacer {
    pub fn new(height: usize) -> Box<Self> {
        Box::new(Self { height })
    }
}

impl<BackendContext> FixedWidth<BackendContext> for VerticalSpacer {
    fn width(&self) -> usize {
        0
    }
}

impl<BackendContext> FixedHeight<BackendContext> for VerticalSpacer {
    fn height(&self) -> usize {
        self.height
    }
}

impl<T> Render<T> for VerticalSpacer {
    fn render(&self, _ctx: &mut T, _top_left: Position) {}
}

/// Spacer that takes up a specified horizontal space.
/// Does not take up any space vertically.
pub struct HorizontalSpacer {
    width: usize,
}

impl HorizontalSpacer {
    pub fn new(width: usize) -> Box<Self> {
        Box::new(Self { width })
    }
}

impl<BackendContext> FixedWidth<BackendContext> for HorizontalSpacer {
    fn width(&self) -> usize {
        self.width
    }
}

impl<BackendContext> FixedHeight<BackendContext> for HorizontalSpacer {
    fn height(&self) -> usize {
        0
    }
}

impl<T> Render<T> for HorizontalSpacer {
    fn render(&self, _ctx: &mut T, _top_left: Position) {}
}
