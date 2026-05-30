use std::marker::PhantomData;

use crate::{
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
};

/// Adds padding around an element
pub struct PaddingContainer<T, BackendContext> {
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    child: Box<T>,
    phantom: PhantomData<BackendContext>,
}

impl<T, BackendContext> FixedWidth<BackendContext> for PaddingContainer<T, BackendContext>
where
    T: FixedWidth<BackendContext>,
{
    fn width(&self) -> usize {
        self.left + self.child.width() + self.right
    }
}

impl<T, BackendContext> FixedHeight<BackendContext> for PaddingContainer<T, BackendContext>
where
    T: FixedHeight<BackendContext>,
{
    fn height(&self) -> usize {
        self.top + self.child.height() + self.bottom
    }
}

impl<T, BackendContext> Render<BackendContext> for PaddingContainer<T, BackendContext>
where
    T: Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: crate::position::Position) {
        self.child
            .render(ctx, top_left + Position::new(self.left, self.top));
    }
}

impl<T, BackendContext> PaddingContainer<T, BackendContext> {
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
    pub fn all(child: Box<T>, padding: usize) -> Box<Self> {
        Box::new(Self {
            left: padding,
            right: padding,
            top: padding,
            bottom: padding,
            child,
            phantom: PhantomData,
        })
    }
}
