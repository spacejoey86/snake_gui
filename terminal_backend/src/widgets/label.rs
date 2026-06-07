use snake_gui::{ElementFixedSizeTrait, Position, widgets::Label};

use crate::PrintBackendCTX;

impl<'a> ElementFixedSizeTrait<'a, PrintBackendCTX, ()> for Label<PrintBackendCTX> {
    fn width(&self) -> usize {
        self.text.len()
    }

    fn height(&self) -> usize {
        1
    }

    fn render(self: Box<Self>, ctx: &mut PrintBackendCTX, top_left: Position) {
        for (i, char) in self.text.chars().enumerate() {
            ctx.buffer[top_left.y][top_left.x + i] = char
        }
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, PrintBackendCTX, ()> + 'b>
    where
        'a: 'b,
    {
        self
    }
}
