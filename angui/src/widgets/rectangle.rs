use std::marker::PhantomData;

use crate::{ElementFixedSize, ElementFixedSizeTrait};

pub struct RectangleElement<BackendContext> {
    pub width: usize,
    pub height: usize,
    pub colour_index: u8,
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static> RectangleElement<BackendContext>
where
    RectangleElement<BackendContext>: ElementFixedSizeTrait<BackendContext, ()>,
{
    pub fn new(
        width: usize,
        height: usize,
        colour_index: u8,
    ) -> ElementFixedSize<BackendContext, ()> {
        ElementFixedSize {
            inner: Box::new(Self {
                width,
                height,
                colour_index,
                phantom: PhantomData,
            }),
        }
    }
}

// Backends should implement ElementFixedSize
