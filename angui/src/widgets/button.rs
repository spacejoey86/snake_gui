pub struct Button {
    pub down: bool,
}

impl Button {
    pub fn new(down: bool) -> Box<Self> {
        Box::new(Self { down })
    }
}

// Backends should implement:
// FixedWidth
// FixedHeight
// Render
