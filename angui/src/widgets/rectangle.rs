use crate::traits::{FixedHeight, FixedWidth};

pub struct RectangleElement {
    pub width: usize,
    pub height: usize,
    pub colour_index: u8,
}

impl RectangleElement {
    pub fn new(width: usize, height: usize, colour_index: u8) -> Box<Self> {
        Box::new(Self { width, height, colour_index })
    }
}

impl<BackendContext> FixedWidth<BackendContext> for RectangleElement {
    fn width(&self) -> usize {
        self.width
    }
}

impl<BackendContext> FixedHeight<BackendContext> for RectangleElement {
    fn height(&self) -> usize {
        self.height
    }
}

// Backends should implement Render
