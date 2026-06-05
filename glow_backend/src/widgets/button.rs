use angui::{
    ElementFixedSizeTrait, Position,
    widgets::{Button, RectangleElement},
};

use crate::GlowBackendContext;

impl ElementFixedSizeTrait<GlowBackendContext> for Button<GlowBackendContext> {
    fn width(&self) -> usize {
        50
    }

    fn height(&self) -> usize {
        30
    }

    fn render(&self, ctx: &mut GlowBackendContext, top_left: angui::Position) {
        let mouse_down_offset = if self.down {
            Position::new(2, 2)
        } else {
            Position::new(0, 0)
        };
        // dark border 1px around top of button
        RectangleElement::new(45, 25, 2).render(ctx, top_left + mouse_down_offset);
        // shadow for bottom of button
        RectangleElement::new(43, 23, 2).render(ctx, top_left + Position::new(7, 7));
        // top of button
        RectangleElement::new(43, 23, 6)
            .render(ctx, top_left + Position::new(2, 2) + mouse_down_offset);
    }
}
