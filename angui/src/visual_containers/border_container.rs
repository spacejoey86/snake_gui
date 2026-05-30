/// Container that draws a border around a child
pub struct BorderContainer<T> {
    pub child: Box<T>,
}

impl<T> BorderContainer<T> {
    pub fn new(child: Box<T>) -> Box<Self> {
        Box::new(Self { child })
    }
}

// Backends should implement for this struct:
// FixedHeight
// FixedWidth
// Render where T: Render + FixedHeight + FixedWidth
