use crate::{GlowBackendContext, Rect};
use angui::{Position, Render, widgets::RectangleElement};

impl Render<GlowBackendContext> for RectangleElement {
    fn render(&self, ctx: &mut GlowBackendContext, top_left: Position) {
        ctx.rects.push(Rect {
            offset_x: (top_left.x as f32 * 2.0) / ctx.window_width as f32 - 1.0,
            offset_y: (top_left.y as f32 * 2.0) / ctx.window_height as f32 - 1.0,
            width: self.width as f32 / ctx.window_width as f32 * 2.0,
            height: self.height as f32 / ctx.window_height as f32 * 2.0,
            colour_index: self.colour_index,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        })
    }
}
