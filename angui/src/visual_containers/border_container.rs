use std::marker::PhantomData;

use crate::{ElementFixedSize, ElementFixedSizeTrait};

/// Container that draws a border around a child
pub struct BorderContainer<BackendContext> {
    pub child: ElementFixedSize<BackendContext>,
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static> BorderContainer<BackendContext>
where
    BorderContainer<BackendContext>: ElementFixedSizeTrait<BackendContext>,
{
    pub fn new(child: ElementFixedSize<BackendContext>) -> ElementFixedSize<BackendContext> {
        ElementFixedSize {
            inner: Box::new(Self {
                child,
                phantom: PhantomData,
            }),
        }
    }
}

// Backends should implement for this struct:
// FixedHeight
// FixedWidth
// Render where T: Render + FixedHeight + FixedWidth
