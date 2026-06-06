use crate::{ElementFixedSize, ElementFixedSizeTrait};

/// Container that draws a border around a child
pub struct BorderContainer<'a, BackendContext, UserState> {
    pub child: ElementFixedSize<'a, BackendContext, UserState>,
}

impl<'a, BackendContext: 'static, UserState: 'static> BorderContainer<'a, BackendContext, UserState>
where
    BorderContainer<'a, BackendContext, UserState>: ElementFixedSizeTrait<'a, BackendContext, UserState>,
{
    pub fn new<T: Into<ElementFixedSize<'a, BackendContext, UserState>>>(
        child: T,
    ) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(Self {
                child: child.into(),
            }),
        }
    }
}

// Backends should implement for this struct:
// FixedHeight
// FixedWidth
// Render where T: Render + FixedHeight + FixedWidth
