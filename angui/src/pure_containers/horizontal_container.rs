use crate::{ElementFixedSize, ElementFixedSizeTrait, ElementFixedWidthGrowingHeight, Position};

pub struct HorizontalContainer<'a, BackendContext, UserState> {
    max_height: usize,
    total_width: usize,
    spacing: usize,
    render: Box<dyn 'a + FnOnce(&mut BackendContext, Position, usize) -> UserState>,
}

impl<'a, BackendContext: 'a, PrevUserState: 'a>
    HorizontalContainer<'a, BackendContext, PrevUserState>
{
    pub fn add_child<
        T: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, ChildUserState>>,
        F: 'a + FnOnce(PrevUserState, ChildUserState) -> UserState,
        ChildUserState: 'a,
        UserState,
    >(
        mut self,
        child: T,
        state_closure: F,
    ) -> HorizontalContainer<'a, BackendContext, UserState> {
        let new_child = child.into();
        let new_child_offset = Position::new(self.total_width + self.spacing, 0);
        self.max_height = self.max_height.max(new_child.min_height());
        self.total_width += self.spacing + new_child.width();

        let new_closure = move |ctx: &mut BackendContext, pos, max_height| {
            let prev_user_state = (self.render)(ctx, pos, max_height);
            let child_user_state = new_child.render(ctx, pos + new_child_offset, max_height);
            (state_closure)(prev_user_state, child_user_state)
        };

        HorizontalContainer {
            max_height: self.max_height,
            total_width: self.total_width,
            spacing: self.spacing,
            render: Box::new(new_closure),
        }
    }
}

impl<'a, BackendContext: 'a, UserState: 'a> ElementFixedSizeTrait<'a, BackendContext, UserState>
    for HorizontalContainer<'a, BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.total_width
    }

    fn height(&self) -> usize {
        self.max_height
    }

    fn render(self: Box<Self>, ctx: &mut BackendContext, top_left: Position) -> UserState {
        (self.render)(ctx, top_left, self.max_height)
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        self
    }
}

impl<'a, BackendContext: 'a, UserState: 'a> Into<ElementFixedSize<'a, BackendContext, UserState>>
    for HorizontalContainer<'a, BackendContext, UserState>
{
    fn into(self) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(self),
        }
    }
}

impl<'a, BackendContext: 'a> HorizontalContainer<'a, BackendContext, ()> {
    pub fn new(spacing: usize) -> Self {
        Self {
            max_height: 0,
            total_width: 0,
            spacing,
            render: Box::new(|_, _, _| ()),
        }
    }
}
