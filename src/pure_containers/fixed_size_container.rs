use crate::traits::{FixedHeight, FixedWidth, Render};

/// container that enforces the child fits within a fixed size
pub struct FixedSizeContainer<T: ?Sized> {
    width: usize,
    height: usize,
    child: Box<T>,
}

impl<T> FixedWidth for FixedSizeContainer<T> {
    fn width(&self) -> usize {
        self.width
    }
}

impl<T> FixedHeight for FixedSizeContainer<T> {
    fn height(&self) -> usize {
        self.height
    }
}

impl<T: ?Sized> FixedSizeContainer<T>
where
    T: FixedWidth + FixedHeight,
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
            }))
        }
    }
}

impl<T: ?Sized, BackendContext> Render<BackendContext> for FixedSizeContainer<T>
where
    T: Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: crate::position::Position) {
        self.child.render(ctx, top_left)
    }
}
