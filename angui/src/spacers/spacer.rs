use std::marker::PhantomData;

use crate::{ElementFixedSize, position::Position, traits::ElementFixedSizeTrait};

/// Spacer that takes up a specified vertical space.
/// Does not take up any space horizontally.
pub struct VerticalSpacer<'a> {
    height: usize,
    phantom: PhantomData<&'a ()>,
}

impl<'a> VerticalSpacer<'a> {
    pub fn new(height: usize) -> Box<Self> {
        Box::new(Self {
            height,
            phantom: PhantomData,
        })
    }

    pub fn covariant<'b>(self) -> VerticalSpacer<'b>
    where
        'a: 'b,
    {
        self
    }

    pub fn covariant_box<'b>(self: Box<Self>) -> Box<VerticalSpacer<'b>>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a, BackendContext> ElementFixedSizeTrait<'a, BackendContext, ()> for VerticalSpacer<'a> {
    fn width(&self) -> usize {
        0
    }

    fn height(&self) -> usize {
        self.height
    }

    fn render(self: Box<Self>, _ctx: &mut BackendContext, _top_left: Position) {}

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, ()> + 'b>
    where
        'a: 'b,
    {
        VerticalSpacer::covariant_box(self)
    }
}

/// Spacer that takes up a specified horizontal space.
/// Does not take up any space vertically.
pub struct HorizontalSpacer<'a, BackendContext> {
    width: usize,
    phantom: PhantomData<(BackendContext, &'a ())>,
}

impl<'a, BackendContext: 'static> HorizontalSpacer<'a, BackendContext> {
    pub fn new(width: usize) -> ElementFixedSize<'a, BackendContext, ()> {
        ElementFixedSize {
            inner: Box::new(Self {
                width,
                phantom: PhantomData,
            }),
        }
    }

    pub fn covariant<'b>(self) -> HorizontalSpacer<'b, BackendContext>
    where
        'a: 'b,
    {
        self
    }

    pub fn covariant_box<'b>(self: Box<Self>) -> Box<HorizontalSpacer<'b, BackendContext>>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a, BackendContext: 'static> ElementFixedSizeTrait<'a, BackendContext, ()>
    for HorizontalSpacer<'a, BackendContext>
{
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        0
    }

    fn render(self: Box<Self>, _ctx: &mut BackendContext, _top_left: Position) {}

    fn covariant_box<'b>(self: Box<Self>) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, ()> + 'b>
    where
        'a: 'b,
    {
        HorizontalSpacer::covariant_box(self)
    }
}
