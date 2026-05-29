use crate::traits::GrowingHeight;

pub struct VerticalSeparator {}

impl GrowingHeight for VerticalSeparator {
    fn min_height(&self) -> usize {
        0
    }
}

impl VerticalSeparator {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
