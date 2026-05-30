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
