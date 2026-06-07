use crate::{ElementFixedSize, position::Position, traits::ElementFixedSizeTrait};

/// Adds padding around an element
pub struct PaddingContainer<'a, BackendContext, UserState> {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    child: ElementFixedSize<'a, BackendContext, UserState>,
}

impl<'a, BackendContext: 'static, UserState: 'static>
    ElementFixedSizeTrait<'a, BackendContext, UserState>
    for PaddingContainer<'a, BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.left + self.child.width() + self.right
    }

    fn height(&self) -> usize {
        self.top + self.child.height() + self.bottom
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: crate::position::Position,
    ) -> UserState {
        self.child
            .render(ctx, top_left + Position::new(self.left, self.top))
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        self.covariant_box()
    }
}

impl<'a, BackendContext: 'static, UserState: 'static>
    PaddingContainer<'a, BackendContext, UserState>
{
    /// Create a padding container, specifying the padding for each side
    pub fn new(
        child: ElementFixedSize<'a, BackendContext, UserState>,
        left: usize,
        right: usize,
        top: usize,
        bottom: usize,
    ) -> Box<Self> {
        Box::new(Self {
            left,
            right,
            top,
            bottom,
            child,
        })
    }

    /// Create a padding container with the same padding on each side
    pub fn all(
        child: ElementFixedSize<'a, BackendContext, UserState>,
        padding: usize,
    ) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(Self {
                left: padding,
                right: padding,
                top: padding,
                bottom: padding,
                child,
            })
            .covariant_box(),
        }
    }

    pub fn covariant<'b>(self) -> PaddingContainer<'b, BackendContext, UserState>
    where
        'a: 'b,
    {
        PaddingContainer {
            left: self.left,
            right: self.right,
            top: self.top,
            bottom: self.bottom,
            child: self.child.covariant(),
        }
    }

    pub fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<PaddingContainer<'b, BackendContext, UserState>>
    where
        'a: 'b,
    {
        Box::new(self.covariant())
    }
}
