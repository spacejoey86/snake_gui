use crate::PrintBackendCTX;
use angui::{ElementFixedSize, ElementFixedSizeTrait, Position};

pub struct CharRectangle {
    width: usize,
    height: usize,
    char: char,
}

impl CharRectangle {
    pub fn new(width: usize, height: usize, char: char) -> ElementFixedSize<PrintBackendCTX, ()> {
        ElementFixedSize {
            inner: Box::new(Self {
                width,
                height,
                char,
            }),
        }
    }
}

impl ElementFixedSizeTrait<PrintBackendCTX, ()> for CharRectangle {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(self: Box<Self>, ctx: &mut PrintBackendCTX, top_left: Position) {
        for y in 0..self.height {
            for x in 0..self.width {
                ctx.buffer[top_left.y + y][top_left.x + x] = self.char;
            }
        }
    }
}
