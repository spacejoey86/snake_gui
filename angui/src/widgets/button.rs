use std::marker::PhantomData;

use crate::{ElementFixedSize, ElementFixedSizeTrait};

pub struct Button<BackendContext> {
    pub down: bool,
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static> Button<BackendContext>
where
    Button<BackendContext>: ElementFixedSizeTrait<BackendContext>,
{
    pub fn new(down: bool) -> ElementFixedSize<BackendContext> {
        ElementFixedSize {
            inner: Box::new(Self {
                down,
                phantom: PhantomData,
            }),
        }
    }
}

// Backends should implement:
// FixedWidth
// FixedHeight
// Render
