use std::marker::PhantomData;

use crate::{ElementFixedSize, traits::ElementFixedSizeTrait};

/// container that enforces the child fits within a fixed size
pub struct FixedSizeContainer<'a, BackendContext, UserState> {
    width: usize,
    height: usize,
    child: ElementFixedSize<'a, BackendContext, UserState>,
    phantom: PhantomData<BackendContext>,
}

impl<'a, BackendContext: 'static, UserState: 'static>
    FixedSizeContainer<'a, BackendContext, UserState>
{
    /// Returns an error if the child doesn't fit within the specified size
    pub fn new(
        width: usize,
        height: usize,
        child: ElementFixedSize<'a, BackendContext, UserState>,
    ) -> Result<ElementFixedSize<'a, BackendContext, UserState>, ()> {
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

    pub fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<FixedSizeContainer<'b, BackendContext, UserState>>
    where
        'a: 'b,
    {
        Box::new(FixedSizeContainer {
            width: self.width,
            height: self.height,
            child: self.child.covariant(),
            phantom: PhantomData,
        })
    }
}

impl<'a, BackendContext: 'static, UserState: 'static>
    ElementFixedSizeTrait<'a, BackendContext, UserState>
    for FixedSizeContainer<'a, BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: crate::position::Position,
    ) -> UserState {
        self.child.render(ctx, top_left)
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
