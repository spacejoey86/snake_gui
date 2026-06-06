use std::marker::PhantomData;

use crate::{ElementFixedSize, ElementFixedSizeTrait};

/// Single line text element
pub struct Label<BackendContext> {
    pub text: String,
    phantom: PhantomData<BackendContext>,
}

impl<'a, BackendContext: 'static> Label<BackendContext>
where
    Label<BackendContext>: ElementFixedSizeTrait<'a, BackendContext, ()>,
{
    pub fn new(text: &str) -> ElementFixedSize<'a, BackendContext, ()> {
        ElementFixedSize {
            inner: Box::new(Self {
                text: text.to_string(),
                phantom: PhantomData,
            }),
        }
    }
}

// Backends should implement:
// FixedHeight
// FixedWidth
// Render
