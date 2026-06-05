use angui::{ElementFixedSizeTrait, Position, widgets::RectangleElement};

use crate::PrintBackendCTX;

impl ElementFixedSizeTrait<PrintBackendCTX, ()> for RectangleElement<PrintBackendCTX> {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(self: Box<Self>, ctx: &mut PrintBackendCTX, top_left: Position) {
        for row in 0..self.height {
            for column in 0..self.width {
                ctx.buffer[top_left.y + row][top_left.x + column] = '█';
            }
        }
    }
}
