use angui::{Position, Render, widgets::RectangleElement};

use crate::PrintBackendCTX;

impl Render<PrintBackendCTX> for RectangleElement {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position) {
        for row in 0..self.height {
            for column in 0..self.width {
                ctx.buffer[top_left.y + row][top_left.x + column] = '█';
            }
        }
    }
}
