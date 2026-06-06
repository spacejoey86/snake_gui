use crate::{
    ElementFixedSize, ElementFixedWidthGrowingHeight, position::Position,
    traits::ElementFixedSizeTrait,
};

/// Place elements one after the other horizontally.
/// Adds spacing between elements.
/// Allows element's height to grow to match the tallest element.
pub struct HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState> {
    left: ElementFixedWidthGrowingHeight<'a, BackendContext, LeftUserState>,
    right: ElementFixedWidthGrowingHeight<'a, BackendContext, RightUserState>,
    state_closure: Box<dyn 'a + FnOnce(LeftUserState, RightUserState) -> UserState>,
    spacing: usize,
}

impl<'a, BackendContext, LeftUserState, RightUserState, UserState>
    HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState>
{
    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<HorizontalContainer<'b, BackendContext, LeftUserState, RightUserState, UserState>>
    where
        'a: 'b,
    {
        Box::new(HorizontalContainer {
            left: self.left.covariant(),
            right: self.right.covariant(),
            state_closure: self.state_closure,
            spacing: self.spacing,
        })
    }
}

impl<
    'a,
    BackendContext: 'static,
    LeftUserState: 'static,
    RightUserState: 'static,
    UserState: 'static,
> ElementFixedSizeTrait<'a, BackendContext, UserState>
    for HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState>
{
    fn width(&self) -> usize {
        self.left.width() + self.spacing + self.right.width()
    }

    fn height(&self) -> usize {
        self.left.min_height().max(self.right.min_height())
    }

    fn render(self: Box<Self>, ctx: &mut BackendContext, top_left: Position) -> UserState {
        let right_offset = self.left.width() + self.spacing;
        let height = self.height();
        let l = self.left.render(ctx, top_left, height);
        let r = self
            .right
            .render(ctx, top_left + Position::new(right_offset, 0), height);

        (self.state_closure)(l, r)
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        self.covariant_box()
    }
}

impl<
    'a,
    BackendContext: 'static,
    InnerLeftUserState: 'static,
    InnerRightUserState: 'static,
    LeftUserState: 'static,
> HorizontalContainer<'a, BackendContext, InnerLeftUserState, InnerRightUserState, LeftUserState>
{
    pub fn add_child<
        T: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, RightUserState>>,
        F: FnOnce(LeftUserState, RightUserState) -> UserState + 'a,
        RightUserState,
        UserState,
    >(
        self,
        child: T,
        state_closure: F,
    ) -> HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState> {
        let spacing = self.spacing;
        HorizontalContainer {
            left: Into::<ElementFixedSize<BackendContext, LeftUserState>>::into(self).into(),
            right: child.into(),
            state_closure: Box::new(state_closure),
            spacing,
        }
    }
}

impl<
    'a,
    BackendContext: 'static,
    LeftUserState: 'static,
    RightUserState: 'static,
    UserState: 'static,
> Into<ElementFixedSize<'a, BackendContext, UserState>>
    for HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState>
{
    fn into(self) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(self).covariant_box(),
        }
    }
}

pub fn horizontal<
    'a,
    BackendContext,
    L: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, LeftUserState>>,
    R: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, RightUserState>>,
    F: FnOnce(LeftUserState, RightUserState) -> UserState + 'a,
    LeftUserState,
    RightUserState,
    UserState,
>(
    spacing: usize,
    left: L,
    right: R,
    state_closure: F,
) -> HorizontalContainer<'a, BackendContext, LeftUserState, RightUserState, UserState> {
    HorizontalContainer {
        left: left.into(),
        right: right.into(),
        state_closure: Box::new(state_closure),
        spacing,
    }
}
