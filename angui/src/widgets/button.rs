use std::marker::PhantomData;

use crate::{ElementFixedSize, ElementFixedSizeTrait};

pub struct Button<BackendContext> {
    pub down: bool,
    phantom: PhantomData<BackendContext>,
}

impl<'a, BackendContext: 'static> Button<BackendContext>
where
    Button<BackendContext>: ElementFixedSizeTrait<'a, BackendContext, ButtonResult>,
{
    pub fn new(down: bool) -> ElementFixedSize<'a, BackendContext, ButtonResult> {
        ElementFixedSize {
            inner: Box::new(Self {
                down,
                phantom: PhantomData,
            }),
        }
    }
}

pub struct ButtonResult {
    /// button was clicked this frame
    pub clicked: bool,
    pub held: bool,
}

// Backends should implement:
// FixedWidth
// FixedHeight
// Render
