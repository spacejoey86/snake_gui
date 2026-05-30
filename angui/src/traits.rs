//! Traits that define UI elements
//! Elements must implement one each of Width, Height and Render traits
//! Blanket implementations are provided for fixed size UI elements to implement the Growable traits

use crate::position::Position;

/// UI element with a width that doesn't grow
pub trait FixedWidth<BackendContext> { // will need to be generic over the backend so that backends can implement it on foreign types?
    fn width(&self) -> usize;
}
/// UI element with a height that doesn't grow
pub trait FixedHeight<BackendContext> {
    fn height(&self) -> usize;
}

/// Render this fixed size UI element
/// Some backends might require an additional call before you see the element on your screen
pub trait Render<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position);
}

// since UI elements are passed around boxed, implement render for a boxed element
impl<T, BackendContext> Render<BackendContext> for Box<T>
where
    T: ?Sized + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        self.as_ref().render(ctx, top_left);
    }
}

/// UI element with a width that can grow
pub trait GrowingWidth<BackendContext> {
    fn min_width(&self) -> usize;
}

// You can use a fixed width UI element as a growable width element
// This doesn't cause the element's width to grow
impl<T, BackendContext> GrowingWidth<BackendContext> for T
where
    T: FixedWidth<BackendContext>,
{
    fn min_width(&self) -> usize {
        self.width()
    }
}

/// UI element with a height that can grow
pub trait GrowingHeight<BackendContext> {
    fn min_height(&self) -> usize;
}

// You can use a fixed height UI element as a growable height element
// This doesn't cause the element's height to grow
impl<T, BackendContext> GrowingHeight<BackendContext> for T
where
    T: FixedHeight<BackendContext>,
{
    fn min_height(&self) -> usize {
        self.height()
    }
}

/// Render a UI element with fixed height, growable width
pub trait RenderGrowWidth<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize);
}

// You can use a fixed width UI element as a growable width element
impl<T, BackendContext> RenderGrowWidth<BackendContext> for T
where
    T: FixedWidth<BackendContext> + Render<BackendContext>, // fixed width bound is just used to do an additional check
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize) {
        assert!(width >= self.width(), "min width not respected");
        self.render(ctx, top_left);
    }
}

/// Render a UI element with a fixed width, growable height
pub trait RenderGrowHeight<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize);
}

// You can use a fixed height UI element as a growable height element
impl<T, BackendContext> RenderGrowHeight<BackendContext> for T
where
    T: FixedHeight<BackendContext> + Render<BackendContext>, // fixed height bound is just used to do an additional check
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize) {
        assert!(height >= self.height(), "min height not respected");
        self.render(ctx, top_left);
    }
}

/// Render a UI element with growable width and height
pub trait RenderGrowBoth<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position);
}

// You can use a UI element with fixed width and height as an element with growable width and height
impl<T, BackendContext> RenderGrowBoth<BackendContext> for T
where
    T: FixedWidth<BackendContext> + FixedHeight<BackendContext> + Render<BackendContext>, // fixed width and height bounds just used to do additional checks
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position) {
        assert!(size.x >= self.width(), "min width not respected");
        assert!(size.y >= self.height(), "min height not respected");
        self.render(ctx, top_left);
    }
}
