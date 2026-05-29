use crate::{
    layout_traits::{FixedWidth, FixedHeight},
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
    fn get_x_size(&self) -> usize {
        self.width
    }
}

impl FixedHeight for RectangleElement {
    fn get_y_size(&self) -> usize {
        self.height
    }
}
