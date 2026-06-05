use std::marker::PhantomData;

use crate::{ElementFixedWidthGrowingHeight, ElementFixedWidthGrowingHeightTrait};

/// Vertical separator bar to provide visual division of elements
pub struct VerticalSeparator<BackendContext> {
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static> VerticalSeparator<BackendContext>
where
    VerticalSeparator<BackendContext>: ElementFixedWidthGrowingHeightTrait<BackendContext, ()>,
{
    pub fn new() -> ElementFixedWidthGrowingHeight<BackendContext, ()> {
        ElementFixedWidthGrowingHeight {
            inner: Box::new(Self {
                phantom: PhantomData,
            }),
        }
    }
}

// Backends should implement:
// FixedWidth
// RenderGrowHeight
// GrowingHeight (see below)

// Backends need to implement this, because of orphan rule
// (Backends could theoretically implement FixedHeight, which has a blanket impl for GrowingHeight)
// impl GrowingHeight<BackendContext> for VerticalSeparator<BackendContext> {
//     fn min_height(&self) -> usize {
//         0
//     }
// }
