use angui::{
    ElementFixedSizeTrait, Position,
    widgets::{Button, ButtonResult, RectangleElement},
};

use crate::GlowBackendContext;

impl<'a> ElementFixedSizeTrait<'a, GlowBackendContext, ButtonResult> for Button<GlowBackendContext> {
    fn width(&self) -> usize {
        50
    }

    fn height(&self) -> usize {
        30
    }

    fn render(
        self: Box<Self>,
        ctx: &mut GlowBackendContext,
        top_left: angui::Position,
    ) -> ButtonResult {
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

        // is the cursor over this element?
        if top_left.inside_top_left(ctx.mouse_pos)
            && (top_left + Position::new(50, 30)).inside_bottom_right(ctx.mouse_pos)
        {
            ctx.mouse_swallowed = true;
            ButtonResult {
                clicked: ctx.mouse_clicked,
                held: ctx.mouse_down,
            }
        } else {
            ButtonResult {
                clicked: false,
                held: false,
            }
        }
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, GlowBackendContext, ButtonResult>>
    where
        'a: 'b,
    {
        self
    }
}
