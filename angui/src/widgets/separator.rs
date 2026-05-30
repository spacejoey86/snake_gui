use std::marker::PhantomData;

/// Vertical separator bar to provide visual division of elements
pub struct VerticalSeparator<BackendContext> {
    phantom: PhantomData<BackendContext>,
}

// Backends need to implement this, because of orphan rule
// (Backends could theoretically implement FixedHeight, which has a blanket impl for GrowingHeight)
// impl GrowingHeight<BackendContext> for VerticalSeparator<BackendContext> {
//     fn min_height(&self) -> usize {
//         0
//     }
// }

impl<BackendContext> VerticalSeparator<BackendContext> {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            phantom: PhantomData,
        })
    }
}
