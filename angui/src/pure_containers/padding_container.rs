use crate::{ElementFixedSize, position::Position, traits::ElementFixedSizeTrait};

/// Adds padding around an element
pub struct PaddingContainer<BackendContext, UserState> {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    child: ElementFixedSize<BackendContext, UserState>,
}

impl<BackendContext, UserState> ElementFixedSizeTrait<BackendContext, UserState>
    for PaddingContainer<BackendContext, UserState>
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
}

impl<BackendContext: 'static, UserState: 'static> PaddingContainer<BackendContext, UserState> {
    /// Create a padding container, specifying the padding for each side
    pub fn new(
        child: ElementFixedSize<BackendContext, UserState>,
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
        child: ElementFixedSize<BackendContext, UserState>,
        padding: usize,
    ) -> ElementFixedSize<BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(Self {
                left: padding,
                right: padding,
                top: padding,
                bottom: padding,
                child,
            }),
        }
    }
}
