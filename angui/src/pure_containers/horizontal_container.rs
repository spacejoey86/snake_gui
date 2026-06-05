use crate::{
    ElementFixedSize, ElementFixedWidthGrowingHeight, position::Position,
    traits::ElementFixedSizeTrait,
};

/// Place elements one after the other horizontally.
/// Adds spacing between elements.
/// Allows element's height to grow to match the tallest element.
pub struct HorizontalContainer<BackendContext, LeftUserState, RightUserState, UserState> {
    left: ElementFixedWidthGrowingHeight<BackendContext, LeftUserState>,
    right: ElementFixedWidthGrowingHeight<BackendContext, RightUserState>,
    state_closure: Box<dyn FnOnce(LeftUserState, RightUserState) -> UserState>,
    spacing: usize,
}

impl<BackendContext, LeftUserState, RightUserState, UserState>
    ElementFixedSizeTrait<BackendContext, UserState>
    for HorizontalContainer<BackendContext, LeftUserState, RightUserState, UserState>
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
}

impl<
    BackendContext: 'static,
    InnerLeftUserState: 'static,
    InnerRightUserState: 'static,
    LeftUserState: 'static,
> HorizontalContainer<BackendContext, InnerLeftUserState, InnerRightUserState, LeftUserState>
{
    pub fn add_child<
        T: Into<ElementFixedWidthGrowingHeight<BackendContext, RightUserState>>,
        F: FnOnce(LeftUserState, RightUserState) -> UserState + 'static,
        RightUserState,
        UserState,
    >(
        self,
        child: T,
        state_closure: F,
    ) -> HorizontalContainer<BackendContext, LeftUserState, RightUserState, UserState> {
        let spacing = self.spacing;
        HorizontalContainer {
            left: Into::<ElementFixedSize<BackendContext, LeftUserState>>::into(self).into(),
            right: child.into(),
            state_closure: Box::new(state_closure),
            spacing,
        }
    }
}

impl<BackendContext: 'static, LeftUserState: 'static, RightUserState: 'static, UserState: 'static>
    Into<ElementFixedSize<BackendContext, UserState>>
    for HorizontalContainer<BackendContext, LeftUserState, RightUserState, UserState>
{
    fn into(self) -> ElementFixedSize<BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(self),
        }
    }
}

pub fn horizontal<
    BackendContext,
    L: Into<ElementFixedWidthGrowingHeight<BackendContext, LeftUserState>>,
    R: Into<ElementFixedWidthGrowingHeight<BackendContext, RightUserState>>,
    F: FnOnce(LeftUserState, RightUserState) -> UserState + 'static,
    LeftUserState,
    RightUserState,
    UserState,
>(
    spacing: usize,
    left: L,
    right: R,
    state_closure: F,
) -> HorizontalContainer<BackendContext, LeftUserState, RightUserState, UserState> {
    HorizontalContainer {
        left: left.into(),
        right: right.into(),
        state_closure: Box::new(state_closure),
        spacing,
    }
}
