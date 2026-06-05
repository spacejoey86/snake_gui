use std::marker::PhantomData;

use crate::{ElementFixedSize, position::Position, traits::ElementFixedSizeTrait};

/// Adds padding around an element
pub struct PaddingContainer<T, BackendContext> {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    child: Box<T>,
    phantom: PhantomData<BackendContext>,
}

impl<T, BackendContext> ElementFixedSizeTrait<BackendContext>
    for PaddingContainer<T, BackendContext>
where
    T: ElementFixedSizeTrait<BackendContext>,
{
    fn width(&self) -> usize {
        self.left + self.child.width() + self.right
    }

    fn height(&self) -> usize {
        self.top + self.child.height() + self.bottom
    }

    fn render(&self, ctx: &mut BackendContext, top_left: crate::position::Position) {
        self.child
            .render(ctx, top_left + Position::new(self.left, self.top));
    }
}

impl<T: 'static, BackendContext: 'static> PaddingContainer<T, BackendContext>
where
    T: ElementFixedSizeTrait<BackendContext>,
{
    /// Create a padding container, specifying the padding for each side
    pub fn new(child: Box<T>, left: usize, right: usize, top: usize, bottom: usize) -> Box<Self> {
        Box::new(Self {
            left,
            right,
            top,
            bottom,
            child,
            phantom: PhantomData,
        })
    }

    /// Create a padding container with the same padding on each side
    pub fn all(child: Box<T>, padding: usize) -> ElementFixedSize<BackendContext> {
        ElementFixedSize {
            inner: Box::new(Self {
                left: padding,
                right: padding,
                top: padding,
                bottom: padding,
                child,
                phantom: PhantomData,
            }),
        }
    }
}
