use crate::{
    backends::print_backend::PrintBackendCTX,
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
    visual_containers::BorderContainer,
};

impl<T> FixedHeight for BorderContainer<T>
where
    T: FixedHeight,
{
    fn height(&self) -> usize {
        self.child.height() + 2
    }
}

impl<T> FixedWidth for BorderContainer<T>
where
    T: FixedWidth,
{
    fn width(&self) -> usize {
        self.child.width() + 2
    }
}

impl<T> Render<PrintBackendCTX> for BorderContainer<T>
where
    T: Render<PrintBackendCTX> + FixedHeight + FixedWidth,
{
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: crate::position::Position) {
        // top and bottom borders
        for x in 0..(self.child.width()) {
            let x_absolute = top_left.x + x + 1;
            ctx.buffer[0][x_absolute] = '─';
            ctx.buffer[self.child.height() + 1][x_absolute] = '─';
        }
        // left and right
        for y in 0..(self.child.height()) {
            let y_absolute = top_left.y + y + 1;
            ctx.buffer[y_absolute][0] = '│';
            ctx.buffer[y_absolute][self.child.width() + 1] = '│';
        }
        // corners
        ctx.buffer[0][0] = '╭';
        ctx.buffer[self.child.height() + 1][0] = '╰';
        ctx.buffer[0][self.child.width() + 1] = '╮';
        ctx.buffer[self.child.height() + 1][self.child.width() + 1] = '╯';
        // child
        self.child.render(ctx, top_left + Position::new(1, 1));
    }
}
