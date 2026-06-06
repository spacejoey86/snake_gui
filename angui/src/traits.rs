//! Traits that define UI elements
//! Elements must implement one of the traits

use crate::position::Position;

/// Fixed size UI element
pub trait ElementFixedSizeTrait<'a, BackendContext, UserState> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    /// Some backends might require an additional call before you see the element on your screen
    fn render(self: Box<Self>, ctx: &mut BackendContext, top_left: Position) -> UserState;
    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b;
}

/// wrapper around the trait, for containers to erase generics of children
pub struct ElementFixedSize<'a, BackendContext, UserState> {
    pub inner: Box<dyn ElementFixedSizeTrait<'a, BackendContext, UserState> + 'a>,
}

impl<'a, BackendContext, UserState> ElementFixedSize<'a, BackendContext, UserState> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }

    pub fn height(&self) -> usize {
        self.inner.height()
    }

    pub fn render(self, ctx: &mut BackendContext, top_left: Position) -> UserState {
        self.inner.render(ctx, top_left)
    }

    pub fn covariant<'b>(self) -> ElementFixedSize<'b, BackendContext, UserState>
    where
        'a: 'b,
    {
        ElementFixedSize {
            inner: self.inner.covariant_box(),
        }
    }

    fn covariant_box<'b>(self: Box<Self>) -> Box<ElementFixedSize<'b, BackendContext, UserState>>
    where
        'a: 'b,
    {
        Box::new(ElementFixedSize::covariant(*self))
    }
}

// You can use a fixed width UI element as a growable width element
impl<'a, BackendContext: 'static, UserState: 'static>
    ElementGrowingWidthFixedHeightTrait<'a, BackendContext, UserState>
    for ElementFixedSize<'a, BackendContext, UserState>
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

    fn covariant<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementGrowingWidthFixedHeightTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        ElementFixedSize::covariant_box::<'b>(self)
    }
}
impl<'a, BackendContext: 'static, UserState: 'static>
    From<ElementFixedSize<'a, BackendContext, UserState>>
    for ElementGrowingWidthFixedHeight<'a, BackendContext, UserState>
{
    fn from(value: ElementFixedSize<'a, BackendContext, UserState>) -> Self {
        ElementGrowingWidthFixedHeight {
            inner: ElementGrowingWidthFixedHeightTrait::covariant::<'a>(
                Box::new(value).covariant_box::<'a>(),
            ),
        }
    }
}

// You can use a fixed height UI element as a growable height element
impl<'a, BackendContext: 'static, UserState: 'static>
    ElementFixedWidthGrowingHeightTrait<'a, BackendContext, UserState>
    for ElementFixedSize<'a, BackendContext, UserState>
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

    fn covariant<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedWidthGrowingHeightTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        ElementFixedSize::covariant_box::<'b>(self)
    }
}
impl<'a, BackendContext: 'static, UserState: 'static>
    From<ElementFixedSize<'a, BackendContext, UserState>>
    for ElementFixedWidthGrowingHeight<'a, BackendContext, UserState>
{
    fn from(value: ElementFixedSize<'a, BackendContext, UserState>) -> Self {
        ElementFixedWidthGrowingHeight {
            inner: ElementFixedWidthGrowingHeightTrait::covariant(Box::new(value)),
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

pub trait ElementGrowingWidthFixedHeightTrait<'a, BackendContext, UserState> {
    fn min_width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        width: usize,
    ) -> UserState;
    fn covariant<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementGrowingWidthFixedHeightTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b;
}

/// wrapper around the trait, to erase generics
pub struct ElementGrowingWidthFixedHeight<'a, BackendContext, UserState> {
    pub inner: Box<dyn ElementGrowingWidthFixedHeightTrait<'a, BackendContext, UserState> + 'a>,
}

impl<'a, BackendContext, UserState> ElementGrowingWidthFixedHeight<'a, BackendContext, UserState> {
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
pub trait ElementFixedWidthGrowingHeightTrait<'a, BackendContext, UserState> {
    fn width(&self) -> usize;
    fn min_height(&self) -> usize;
    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: Position,
        height: usize,
    ) -> UserState;
    fn covariant<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedWidthGrowingHeightTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b;
}

/// wrapper to erase generics
pub struct ElementFixedWidthGrowingHeight<'a, BackendContext, UserState> {
    pub inner: Box<dyn ElementFixedWidthGrowingHeightTrait<'a, BackendContext, UserState> + 'a>,
}

impl<'a, BackendContext, UserState> ElementFixedWidthGrowingHeight<'a, BackendContext, UserState> {
    pub fn width(&self) -> usize {
        self.inner.width()
    }
    pub fn min_height(&self) -> usize {
        self.inner.min_height()
    }
    pub fn render(self, ctx: &mut BackendContext, top_left: Position, height: usize) -> UserState {
        self.inner.render(ctx, top_left, height)
    }

    pub fn covariant<'b>(self) -> ElementFixedWidthGrowingHeight<'b, BackendContext, UserState>
    where
        'a: 'b,
    {
        ElementFixedWidthGrowingHeight {
            inner: self.inner.covariant(),
        }
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
