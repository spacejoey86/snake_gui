use angui::{
    position::Position,
    widgets::VerticalSeparator,
    {FixedWidth, RenderGrowHeight},
};

use crate::PrintBackendCTX;

impl FixedWidth<PrintBackendCTX> for VerticalSeparator<PrintBackendCTX> {
    fn width(&self) -> usize {
        1
    }
}

impl RenderGrowHeight<PrintBackendCTX> for VerticalSeparator<PrintBackendCTX> {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position, height: usize) {
        for y in 0..height {
            ctx.buffer[top_left.y + y][top_left.x] = '│';
        }
    }
}
