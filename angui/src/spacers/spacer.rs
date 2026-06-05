use crate::{position::Position, traits::ElementFixedSizeTrait};

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

impl<T> ElementFixedSizeTrait<T> for VerticalSpacer {
    fn width(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.height
    }

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

impl<T> ElementFixedSizeTrait<T> for HorizontalSpacer {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        0
    }

    fn render(&self, _ctx: &mut T, _top_left: Position) {}
}
