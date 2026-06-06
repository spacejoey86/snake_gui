use angui::{ElementFixedWidthGrowingHeightTrait, Position, widgets::VerticalSeparator};

use crate::PrintBackendCTX;

impl<'a> ElementFixedWidthGrowingHeightTrait<'a, PrintBackendCTX, ()>
    for VerticalSeparator<PrintBackendCTX>
{
    fn width(&self) -> usize {
        1
    }

    fn min_height(&self) -> usize {
        0
    }

    fn render(self: Box<Self>, ctx: &mut PrintBackendCTX, top_left: Position, height: usize) {
        for y in 0..height {
            ctx.buffer[top_left.y + y][top_left.x] = '│';
        }
    }

    fn covariant<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedWidthGrowingHeightTrait<'b, PrintBackendCTX, ()> + 'b>
    where
        'a: 'b,
    {
        self
    }
}
