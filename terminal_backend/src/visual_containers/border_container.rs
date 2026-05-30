use crate::PrintBackendCTX;
use angui::{
    Position,
    visual_containers::BorderContainer,
    {FixedHeight, FixedWidth, Render},
};

impl<T> FixedHeight<PrintBackendCTX> for BorderContainer<T>
where
    T: FixedHeight<PrintBackendCTX>,
{
    fn height(&self) -> usize {
        self.child.height() + 2
    }
}

impl<T> FixedWidth<PrintBackendCTX> for BorderContainer<T>
where
    T: FixedWidth<PrintBackendCTX>,
{
    fn width(&self) -> usize {
        self.child.width() + 2
    }
}

impl<T> Render<PrintBackendCTX> for BorderContainer<T>
where
    T: Render<PrintBackendCTX> + FixedHeight<PrintBackendCTX> + FixedWidth<PrintBackendCTX>,
{
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: Position) {
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
