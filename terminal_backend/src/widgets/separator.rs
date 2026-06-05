use angui::{ElementFixedWidthGrowingHeightTrait, Position, widgets::VerticalSeparator};

use crate::PrintBackendCTX;

impl ElementFixedWidthGrowingHeightTrait<PrintBackendCTX> for VerticalSeparator<PrintBackendCTX> {
    fn width(&self) -> usize {
        1
    }

    fn min_height(&self) -> usize {
        0
    }

    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position, height: usize) {
        for y in 0..height {
            ctx.buffer[top_left.y + y][top_left.x] = '│';
        }
    }
}
