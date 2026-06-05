use angui::{FixedHeight, FixedWidth, Position, ElementFixedSizeTrait, widgets::Label};

use crate::PrintBackendCTX;

impl FixedHeight<PrintBackendCTX> for Label {
    fn height(&self) -> usize {
        1
    }
}

impl FixedWidth<PrintBackendCTX> for Label {
    fn width(&self) -> usize {
        self.text.len()
    }
}

impl ElementFixedSizeTrait<PrintBackendCTX> for Label {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position) {
        for (i, char) in self.text.chars().enumerate() {
            ctx.buffer[top_left.y][top_left.x + i] = char
        }
    }
}
