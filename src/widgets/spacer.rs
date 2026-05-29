use crate::{
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
};

pub struct VerticalSpacer {
    height: usize,
}

impl VerticalSpacer {
    pub fn new(height: usize) -> Box<Self> {
        Box::new(Self { height })
    }
}

impl FixedWidth for VerticalSpacer {
    fn width(&self) -> usize {
        0
    }
}

impl FixedHeight for VerticalSpacer {
    fn height(&self) -> usize {
        self.height
    }
}

impl<T> Render<T> for VerticalSpacer {
    fn render(&self, _ctx: &mut T, _top_left: Position) {}
}

pub struct HorizontalSpacer {
    width: usize,
}

impl HorizontalSpacer {
    pub fn new(width: usize) -> Box<Self> {
        Box::new(Self { width })
    }
}

impl FixedWidth for HorizontalSpacer {
    fn width(&self) -> usize {
        self.width
    }
}

impl FixedHeight for HorizontalSpacer {
    fn height(&self) -> usize {
        0
    }
}

impl<T> Render<T> for HorizontalSpacer {
    fn render(&self, _ctx: &mut T, _top_left: Position) {}
}
