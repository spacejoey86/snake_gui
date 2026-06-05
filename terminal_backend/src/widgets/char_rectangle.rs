use crate::PrintBackendCTX;
use angui::{FixedHeight, FixedWidth, Position, ElementFixedSizeTrait};

pub struct CharRectangle {
    width: usize,
    height: usize,
    char: char,
}

impl CharRectangle {
    pub fn new(width: usize, height: usize, char: char) -> Box<Self> {
        Box::new(Self {
            width,
            height,
            char,
        })
    }
}

impl ElementFixedSizeTrait<PrintBackendCTX> for CharRectangle {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position) {
        for y in 0..self.height {
            for x in 0..self.width {
                ctx.buffer[top_left.y + y][top_left.x + x] = self.char;
            }
        }
    }
}

impl FixedWidth<PrintBackendCTX> for CharRectangle {
    fn width(&self) -> usize {
        self.width
    }
}

impl FixedHeight<PrintBackendCTX> for CharRectangle {
    fn height(&self) -> usize {
        self.height
    }
}
