use crate::traits::{FixedHeight, FixedWidth};

pub struct RectangleElement {
    pub width: usize,
    pub height: usize,
}

impl RectangleElement {
    pub fn new(width: usize, height: usize) -> Box<Self> {
        Box::new(Self { width, height })
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
