use crate::layout_traits::{KnownXSizeElement, KnownYSizeElement};

pub struct RectangleElement {
    width: usize,
    height: usize,
}

impl RectangleElement {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl KnownXSizeElement for RectangleElement {
    fn get_x_size(&self) -> usize {
        self.width
    }
}

impl KnownYSizeElement for RectangleElement {
    fn get_y_size(&self) -> usize {
        self.height
    }
}
