use snake_gui::{ElementFixedSizeTrait, Position, widgets::Label};

use crate::{FONT_CHARS, FONT_NUM_CHARACTERS, GlowBackendContext, Rect, font_data};

impl<'a> ElementFixedSizeTrait<'a, GlowBackendContext, ()> for Label<GlowBackendContext> {
    fn width(&self) -> usize {
        font_data().1 as usize / FONT_NUM_CHARACTERS * self.text.len()
    }

    fn height(&self) -> usize {
        font_data().2 as usize
    }

    fn render(self: Box<Self>, ctx: &mut GlowBackendContext, top_left: Position) {
        let mut x_offset = 0;
        let char_width = font_data().1 as usize / FONT_NUM_CHARACTERS;
        let char_height = 16;
        for char in self.text.chars() {
            if char != ' ' {
                let char_index = FONT_CHARS
                    .find(char)
                    .map(
                        |i| i + 2, // two special characters
                    )
                    .unwrap_or(1); // index 1 is a box, to replace characters not in this font
                let x = top_left.x + (x_offset * char_width);
                ctx.rects.push(Rect {
                    offset_x: (x as f32 * 2.0) / ctx.window_width as f32 - 1.0,
                    offset_y: (top_left.y as f32 * 2.0) / ctx.window_height as f32 - 1.0,
                    width: char_width as f32 / ctx.window_width as f32 * 2.0,
                    height: char_height as f32 / ctx.window_height as f32 * 2.0,
                    colour_index: 1,
                    texture_offset_x: char_index as f32 / FONT_NUM_CHARACTERS as f32,
                    texture_offset_y: 0.0,
                })
            }
            x_offset += 1;
            // todo: add spacing between characters
        }
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, GlowBackendContext, ()> + 'b>
    where
        'a: 'b,
    {
        self
    }
}
