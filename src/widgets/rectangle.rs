use crate::{
    traits::{FixedWidth, FixedHeight},
};

pub struct RectangleElement {
    pub width: usize,
    pub height: usize,
}

impl RectangleElement {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl FixedWidth for RectangleElement {
    fn width(&self) -> usize {
        self.width
    }
}

impl FixedHeight for RectangleElement {
    fn height(&self) -> usize {
        self.height
    }
}
