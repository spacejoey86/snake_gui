use std::marker::PhantomData;

use crate::{ElementFixedSize, position::Position, traits::ElementFixedSizeTrait};

/// Spacer that takes up a specified vertical space.
/// Does not take up any space horizontally.
pub struct VerticalSpacer {
    height: usize,
}

impl VerticalSpacer {
    pub fn new(height: usize) -> Box<Self> {
        Box::new(Self { height })
    }
}

impl<BackendContext> ElementFixedSizeTrait<BackendContext, ()> for VerticalSpacer {
    fn width(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(self: Box<Self>, _ctx: &mut BackendContext, _top_left: Position) {}
}

/// Spacer that takes up a specified horizontal space.
/// Does not take up any space vertically.
pub struct HorizontalSpacer<BackendContext> {
    width: usize,
    phantom: PhantomData<BackendContext>,
}

impl<BackendContext: 'static> HorizontalSpacer<BackendContext> {
    pub fn new(width: usize) -> ElementFixedSize<BackendContext, ()> {
        ElementFixedSize {
            inner: Box::new(Self {
                width,
                phantom: PhantomData,
            }),
        }
    }
}

impl<BackendContext> ElementFixedSizeTrait<BackendContext, ()>
    for HorizontalSpacer<BackendContext>
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        0
    }

    fn render(self: Box<Self>, _ctx: &mut BackendContext, _top_left: Position) {}
}
