use snake_gui::{
    ElementFixedSizeTrait, Position,
    widgets::{RectangleElement, Slider, SliderResult},
};

use crate::GlowBackendContext;

impl<'a> ElementFixedSizeTrait<'a, GlowBackendContext, SliderResult>
    for Slider<GlowBackendContext>
{
    fn width(&self) -> usize {
        100
    }

    fn height(&self) -> usize {
        10
    }

    fn render(
        self: Box<Self>,
        ctx: &mut GlowBackendContext,
        top_left: snake_gui::Position,
    ) -> SliderResult {
        let handle_size = Position::new(6, 10);
        let width = 100;

        // slider rail
        RectangleElement::new(width, 6, 6).render(ctx, top_left + Position::new(0, 2));
        // top of slider
        let handle_pos = top_left + Position::new(self.val as usize, 0);
        RectangleElement::new(handle_size.x, handle_size.y, 2).render(ctx, handle_pos);

        let mut clicked = false;
        let mut val = self.val;

        // is the cursor over the slider?
        if top_left.inside_top_left(ctx.mouse_pos)
            && (handle_pos + Position::new(100, 10)).inside_bottom_right(ctx.mouse_pos)
        {
            ctx.mouse_swallowed = true;
            if ctx.mouse_clicked {
                clicked = true
            }
        }

        if self.clicked && ctx.mouse_down {
            // the current drag started with this element
            clicked = true;
            ctx.mouse_swallowed = true;
            val = (ctx
                .mouse_pos
                .x
                .clamp(top_left.x, top_left.x + width - handle_size.x)
                - top_left.x) as u32
        }

        SliderResult { clicked, val }
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, GlowBackendContext, SliderResult> + 'b>
    where
        'a: 'b,
    {
        self
    }
}
