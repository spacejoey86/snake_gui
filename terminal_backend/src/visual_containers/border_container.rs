use crate::PrintBackendCTX;
use snake_gui::{ElementFixedSizeTrait, Position, visual_containers::BorderContainer};

impl<'a, UserState: 'static> ElementFixedSizeTrait<'a, PrintBackendCTX, UserState>
    for BorderContainer<'a, PrintBackendCTX, UserState>
{
    fn width(&self) -> usize {
        self.child.width() + 2
    }

    fn height(&self) -> usize {
        self.child.height() + 2
    }

    fn render(self: Box<Self>, ctx: &mut PrintBackendCTX, top_left: Position) -> UserState {
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
        self.child.render(ctx, top_left + Position::new(1, 1))
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, PrintBackendCTX, UserState> + 'b>
    where
        'a: 'b,
    {
        Box::new(BorderContainer {
            child: self.child.covariant(),
        })
    }
}
