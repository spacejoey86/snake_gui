use std::marker::PhantomData;

use crate::traits::{FixedHeight, FixedWidth, Render};

/// container that enforces the child fits within a fixed size
pub struct FixedSizeContainer<T: ?Sized, BackendContext> {
    width: usize,
    height: usize,
    child: Box<T>,
    phantom: PhantomData<BackendContext>,
}

impl<T, BackendContext> FixedWidth<BackendContext> for FixedSizeContainer<T, BackendContext> {
    fn width(&self) -> usize {
        self.width
    }
}

impl<T, BackendContext> FixedHeight<BackendContext> for FixedSizeContainer<T, BackendContext> {
    fn height(&self) -> usize {
        self.height
    }
}

impl<T: ?Sized, BackendContext> FixedSizeContainer<T, BackendContext>
where
    T: FixedWidth<BackendContext> + FixedHeight<BackendContext>,
{
    /// Returns an error if the child doesn't fit within the specified size
    pub fn new(width: usize, height: usize, child: Box<T>) -> Result<Box<Self>, ()> {
        if width < child.width() || height < child.height() {
            Err(())
        } else {
            Ok(Box::new(Self {
                width,
                height,
                child,
                phantom: PhantomData,
            }))
        }
    }
}

impl<T: ?Sized, BackendContext> Render<BackendContext> for FixedSizeContainer<T, BackendContext>
where
    T: Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: crate::position::Position) {
        self.child.render(ctx, top_left)
    }
}
