//! Traits that define UI elements
//! Elements must implement one of the traits
//! todo: Blanket implementations for fixed size UI elements to implement the Growable traits

use crate::position::Position;

/// Fixed size UI element
pub trait ElementFixedSizeTrait<BackendContext> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Some backends might require an additional call before you see the element on your screen
    fn render(&self, ctx: &mut BackendContext, top_left: Position);
}

/// wrapper around the trait, to erase generics
pub struct ElementFixedSize<BackendContext> {
    pub inner: Box<dyn ElementFixedSizeTrait<BackendContext>>,
}

impl<BackendContext> ElementFixedSize<BackendContext> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }

    pub fn height(&self) -> usize {
        self.inner.height()
    }

    pub fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        self.inner.render(ctx, top_left)
    }
}

// You can use a fixed width UI element as a growable width element
impl<BackendContext> ElementGrowingWidthFixedHeightTrait<BackendContext>
    for ElementFixedSize<BackendContext>
{
    fn min_width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn render(&self, ctx: &mut BackendContext, top_left: Position, _width: usize) {
        self.render(ctx, top_left);
    }
}
impl<BackendContext: 'static> From<ElementFixedSize<BackendContext>>
    for ElementGrowingWidthFixedHeight<BackendContext>
{
    fn from(value: ElementFixedSize<BackendContext>) -> Self {
        ElementGrowingWidthFixedHeight {
            inner: Box::new(value),
        }
    }
}

// You can use a fixed height UI element as a growable height element
impl<BackendContext> ElementFixedWidthGrowingHeightTrait<BackendContext>
    for ElementFixedSize<BackendContext>
{
    fn width(&self) -> usize {
        self.width()
    }

    fn min_height(&self) -> usize {
        self.height()
    }

    fn render(&self, ctx: &mut BackendContext, top_left: Position, _height: usize) {
        self.render(ctx, top_left);
    }
}
impl<BackendContext: 'static> From<ElementFixedSize<BackendContext>>
    for ElementFixedWidthGrowingHeight<BackendContext>
{
    fn from(value: ElementFixedSize<BackendContext>) -> Self {
        ElementFixedWidthGrowingHeight {
            inner: Box::new(value),
        }
    }
}
// This doesn't cause the element's height to grow
// impl<T, BackendContext> GrowingHeight<BackendContext> for T
// where
// T: FixedHeight<BackendContext>,
// {
//     fn min_height(&self) -> usize {
//         self.height()
//     }
// }

pub trait ElementGrowingWidthFixedHeightTrait<BackendContext> {
    fn min_width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize);
}

/// wrapper around the trait, to erase generics
pub struct ElementGrowingWidthFixedHeight<BackendContext> {
    pub inner: Box<dyn ElementGrowingWidthFixedHeightTrait<BackendContext>>,
}

impl<BackendContext> ElementGrowingWidthFixedHeight<BackendContext> {
    pub fn min_width(&self) -> usize {
        self.inner.min_width()
    }
    pub fn height(&self) -> usize {
        self.inner.height()
    }
    pub fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize) {
        self.inner.render(ctx, top_left, width);
    }
}

// You can use a fixed width UI element as a growable width element
// impl<T, BackendContext> ElementGrowingWidthFixedHeight<BackendContext> for T
// where
// T: FixedWidth<BackendContext> + ElementFixedSize<BackendContext>, // fixed width bound is just used to do an additional check
// {
//     fn render(&self, ctx: &mut BackendContext, top_left: Position, width: usize) {
//         assert!(width >= self.width(), "min width not respected");
//         self.render(ctx, top_left);
//     }
// }

/// Render a UI element with a fixed width, growable height
pub trait ElementFixedWidthGrowingHeightTrait<BackendContext> {
    fn width(&self) -> usize;
    fn min_height(&self) -> usize;
    fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize);
}

/// wrapper to erase generics
pub struct ElementFixedWidthGrowingHeight<BackendContext> {
    pub inner: Box<dyn ElementFixedWidthGrowingHeightTrait<BackendContext>>,
}

impl<BackendContext> ElementFixedWidthGrowingHeight<BackendContext> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }
    pub fn min_height(&self) -> usize {
        self.inner.min_height()
    }
    pub fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize) {
        self.inner.render(ctx, top_left, height)
    }
}

// You can use a fixed height UI element as a growable height element
// impl<T, BackendContext> RenderGrowHeight<BackendContext> for T
// where
//     T: FixedHeight<BackendContext> + ElementFixedSize<BackendContext>, // fixed height bound is just used to do an additional check
// {
//     fn render(&self, ctx: &mut BackendContext, top_left: Position, height: usize) {
//         assert!(height >= self.height(), "min height not respected");
//         self.render(ctx, top_left);
//     }
// }

pub trait ElementGrowingSizeTrait<BackendContext> {
    fn min_width(&self) -> usize;
    fn min_height(&self) -> usize;
    fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position);
}

/// wrapper to erase generics
pub struct ElementGrowingSize<BackendContext> {
    pub inner: Box<dyn ElementGrowingSizeTrait<BackendContext>>,
}

impl<BackendContext> ElementGrowingSize<BackendContext> {
    pub fn min_width(&self) -> usize {
        self.inner.min_width()
    }
    pub fn min_height(&self) -> usize {
        self.inner.min_height()
    }
    pub fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position) {
        self.inner.render(ctx, top_left, size)
    }
}

// You can use a UI element with fixed width and height as an element with growable width and height
// impl<T, BackendContext> RenderGrowBoth<BackendContext> for T
// where
//     T: FixedWidth<BackendContext> + FixedHeight<BackendContext> + ElementFixedSize<BackendContext>, // fixed width and height bounds just used to do additional checks
// {
//     fn render(&self, ctx: &mut BackendContext, top_left: Position, size: Position) {
//         assert!(size.x >= self.width(), "min width not respected");
//         assert!(size.y >= self.height(), "min height not respected");
//         self.render(ctx, top_left);
//     }
// }
