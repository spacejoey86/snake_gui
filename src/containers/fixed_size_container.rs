use crate::layout_traits::{KnownXSizeElement, KnownYSizeElement, Render};

/// container that enforces the child fits within a fixed size
pub struct FixedSizeContainer<T: ?Sized> {
    width: usize,
    height: usize,
    child: Box<T>,
}

impl<T> KnownXSizeElement for FixedSizeContainer<T> {
    fn get_x_size(&self) -> usize {
        self.width
    }
}

impl<T> KnownYSizeElement for FixedSizeContainer<T> {
    fn get_y_size(&self) -> usize {
        self.height
    }
}

impl<T: ?Sized> FixedSizeContainer<T>
where
    T: KnownXSizeElement + KnownYSizeElement,
{
    pub fn new(width: usize, height: usize, child: Box<T>) -> Result<Self, ()> {
        if width < child.get_x_size() || height < child.get_y_size() {
            Err(())
        } else {
            Ok(Self {
                width,
                height,
                child,
            })
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
