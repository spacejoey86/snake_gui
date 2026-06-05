use crate::{ElementFixedSize, ElementFixedSizeTrait};

/// Container that draws a border around a child
pub struct BorderContainer<BackendContext, UserState> {
    pub child: ElementFixedSize<BackendContext, UserState>,
}

impl<BackendContext: 'static, UserState: 'static> BorderContainer<BackendContext, UserState>
where
    BorderContainer<BackendContext, UserState>: ElementFixedSizeTrait<BackendContext, UserState>,
{
    pub fn new<T: Into<ElementFixedSize<BackendContext, UserState>>>(
        child: T,
    ) -> ElementFixedSize<BackendContext, UserState> {
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
