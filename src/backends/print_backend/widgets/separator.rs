use crate::{
    backends::print_backend::PrintBackendCTX,
    traits::{FixedWidth, RenderGrowHeight},
    widgets::VerticalSeparator,
};

impl FixedWidth for VerticalSeparator {
    fn width(&self) -> usize {
        1
    }
}

impl RenderGrowHeight<PrintBackendCTX> for VerticalSeparator {
    fn render(
        &self,
        ctx: &mut PrintBackendCTX,
        top_left: crate::position::Position,
        height: usize,
    ) {
        for y in 0..height {
            ctx.buffer[top_left.y + y][top_left.x] = '│';
        }
    }
}
