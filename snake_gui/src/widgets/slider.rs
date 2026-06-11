use std::marker::PhantomData;

use crate::{
    ElementFixedSize, ElementFixedSizeTrait,
};

pub struct Slider<BackendContext> {
    pub clicked: bool, // the mouse is still down after clicking over this slider
    pub val: u32,
    phantom: PhantomData<BackendContext>,
}

impl<'a, BackendContext: 'a> Slider<BackendContext>
where
    Slider<BackendContext>: ElementFixedSizeTrait<'a, BackendContext, SliderResult>,
{
    pub fn new(clicked: bool, val: u32) -> ElementFixedSize<'a, BackendContext, SliderResult> {
        ElementFixedSize {
            inner: Box::new(Self {
                clicked,
                val,
                phantom: PhantomData,
            }),
        }
    }
}

pub struct SliderResult {
    pub clicked: bool,
    pub val: u32,
}

// backends should implement ElementGrowingWidthFixedHeightTrait
