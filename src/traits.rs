use crate::position::Position;

/// a ui element that can calculate it's X size independently of parents
/// will need to be generic over the backend so that backends can implement it on foreign types?
pub trait FixedWidth {
    fn width(&self) -> usize;
}
/// a ui element that can calculate it's Y size independently of parents
pub trait FixedHeight {
    fn height(&self) -> usize;
}

/// like egui Widget trait
/// given sizes (?) render yourself?
/// for a GUI on a screen, the top level has a size
/// but some things might not - windows which can resize, document of arbitrary size
/// does this actually render, or just layout
/// depends on your backend?
pub trait Render<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position);
}

impl<T, BackendContext> Render<BackendContext> for Box<T>
where
    T: ?Sized + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        self.as_ref().render(ctx, top_left);
    }
}

pub trait GrowingWidth {
    fn min_width(&self) -> usize;
}

impl<T> GrowingWidth for T
where
    T: FixedWidth,
{
    fn min_width(&self) -> usize {
        self.width()
    }
}

pub trait GrowingHeight {
    fn min_height(&self) -> usize;
}

impl<T> GrowingHeight for T
where
    T: FixedHeight,
{
    fn min_height(&self) -> usize {
        self.height()
    }
}

pub trait RenderGrowWidth<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize);
}

impl<T, BackendContext> RenderGrowWidth<BackendContext> for T
where
    T: FixedWidth + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize) {
        assert!(width >= self.width(), "min width not respected");
        self.render(ctx, top_left);
    }
}

pub trait RenderGrowHeight<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize);
}

impl<T, BackendContext> RenderGrowHeight<BackendContext> for T
where
    T: FixedHeight + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize) {
        assert!(height >= self.height(), "min height not respected");
        self.render(ctx, top_left);
    }
}

pub trait RenderGrowBoth<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position);
}

impl<T, BackendContext> RenderGrowBoth<BackendContext> for T
where
    T: FixedWidth + FixedHeight + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position) {
        assert!(size.x >= self.width(), "min width not respected");
        assert!(size.y >= self.height(), "min height not respected");
        self.render(ctx, top_left);
    }
}
