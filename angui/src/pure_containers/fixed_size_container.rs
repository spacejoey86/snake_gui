use std::marker::PhantomData;

use crate::{ElementFixedSize, traits::ElementFixedSizeTrait};

/// container that enforces the child fits within a fixed size
pub struct FixedSizeContainer<BackendContext, UserState> {
    width: usize,
    height: usize,
    child: ElementFixedSize<BackendContext, UserState>,
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static, UserState: 'static> FixedSizeContainer<BackendContext, UserState> {
    /// Returns an error if the child doesn't fit within the specified size
    pub fn new(
        width: usize,
        height: usize,
        child: ElementFixedSize<BackendContext, UserState>,
    ) -> Result<ElementFixedSize<BackendContext, UserState>, ()> {
        if width < child.width() || height < child.height() {
            Err(())
        } else {
            Ok(ElementFixedSize {
                inner: Box::new(Self {
                    width,
                    height,
                    child,
                    phantom: PhantomData,
                }),
            })
        }
    }
}

impl<BackendContext, UserState> ElementFixedSizeTrait<BackendContext, UserState>
    for FixedSizeContainer<BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(self: Box<Self>, ctx: &mut BackendContext, top_left: crate::position::Position) -> UserState {
        self.child.render(ctx, top_left)
    }
}
