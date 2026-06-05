//! Traits that define UI elements
//! Elements must implement one of the traits
//! todo: Blanket implementations for fixed size UI elements to implement the Growable traits

use crate::position::Position;

/// Fixed size UI element
pub trait ElementFixedSizeTrait<BackendContext, UserState> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Some backends might require an additional call before you see the element on your screen
    fn render(self: Box<Self>, ctx: &mut BackendContext, top_left: Position) -> UserState;
}

/// wrapper around the trait, to erase generics
/// its not actually erasing any generics, do I need this?
pub struct ElementFixedSize<BackendContext, UserState> {
    pub inner: Box<dyn ElementFixedSizeTrait<BackendContext, UserState>>,
}

impl<BackendContext, UserState> ElementFixedSize<BackendContext, UserState> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }

    pub fn height(&self) -> usize {
        self.inner.height()
    }

    pub fn render(self, ctx: &mut BackendContext, top_left: Position) -> UserState {
        self.inner.render(ctx, top_left)
    }
}

// You can use a fixed width UI element as a growable width element
impl<BackendContext, UserState> ElementGrowingWidthFixedHeightTrait<BackendContext, UserState>
    for ElementFixedSize<BackendContext, UserState>
{
    fn min_width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        _width: usize,
    ) -> UserState {
        ElementFixedSize::render(*self, ctx, top_left)
    }
}
impl<BackendContext: 'static, UserState: 'static> From<ElementFixedSize<BackendContext, UserState>>
    for ElementGrowingWidthFixedHeight<BackendContext, UserState>
{
    fn from(value: ElementFixedSize<BackendContext, UserState>) -> Self {
        ElementGrowingWidthFixedHeight {
            inner: Box::new(value),
        }
    }
}

// You can use a fixed height UI element as a growable height element
impl<BackendContext, UserState> ElementFixedWidthGrowingHeightTrait<BackendContext, UserState>
    for ElementFixedSize<BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.width()
    }

    fn min_height(&self) -> usize {
        self.height()
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        _height: usize,
    ) -> UserState {
        ElementFixedSize::render(*self, ctx, top_left)
    }
}
impl<BackendContext: 'static, UserState: 'static> From<ElementFixedSize<BackendContext, UserState>>
    for ElementFixedWidthGrowingHeight<BackendContext, UserState>
{
    fn from(value: ElementFixedSize<BackendContext, UserState>) -> Self {
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

pub trait ElementGrowingWidthFixedHeightTrait<BackendContext, UserState> {
    fn min_width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        width: usize,
    ) -> UserState;
}

/// wrapper around the trait, to erase generics
pub struct ElementGrowingWidthFixedHeight<BackendContext, UserState> {
    pub inner: Box<dyn ElementGrowingWidthFixedHeightTrait<BackendContext, UserState>>,
}

impl<BackendContext, UserState> ElementGrowingWidthFixedHeight<BackendContext, UserState> {
    pub fn min_width(&self) -> usize {
        self.inner.min_width()
    }
    pub fn height(&self) -> usize {
        self.inner.height()
    }
    pub fn render(self, ctx: &mut BackendContext, top_left: Position, width: usize) -> UserState {
        self.inner.render(ctx, top_left, width)
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
pub trait ElementFixedWidthGrowingHeightTrait<BackendContext, UserState> {
    fn width(&self) -> usize;
    fn min_height(&self) -> usize;
    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        height: usize,
    ) -> UserState;
}

/// wrapper to erase generics
pub struct ElementFixedWidthGrowingHeight<BackendContext, UserState> {
    pub inner: Box<dyn ElementFixedWidthGrowingHeightTrait<BackendContext, UserState>>,
}

impl<BackendContext, UserState> ElementFixedWidthGrowingHeight<BackendContext, UserState> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }
    pub fn min_height(&self) -> usize {
        self.inner.min_height()
    }
    pub fn render(self, ctx: &mut BackendContext, top_left: Position, height: usize) -> UserState {
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

pub trait ElementGrowingSizeTrait<BackendContext, UserState> {
    fn min_width(&self) -> usize;
    fn min_height(&self) -> usize;
    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        size: Position,
    ) -> UserState;
}

/// wrapper to erase generics
pub struct ElementGrowingSize<BackendContext, UserState> {
    pub inner: Box<dyn ElementGrowingSizeTrait<BackendContext, UserState>>,
}

impl<BackendContext, UserState> ElementGrowingSize<BackendContext, UserState> {
    pub fn min_width(&self) -> usize {
        self.inner.min_width()
    }
    pub fn min_height(&self) -> usize {
        self.inner.min_height()
    }
    pub fn render(self, ctx: &mut BackendContext, top_left: Position, size: Position) -> UserState {
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
