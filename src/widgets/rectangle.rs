use crate::{
    backends::print_backend::PrintBackendCTX,
    layout_traits::{KnownXSizeElement, KnownYSizeElement, Render},
};

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

impl Render<PrintBackendCTX> for RectangleElement {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: crate::position::Position) {
        for row in 0..self.height {
            for column in 0..self.width {
                ctx.buffer[top_left.y + row][top_left.x + column] = '█';
            }
        }
    }
}
