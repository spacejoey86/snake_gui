use crate::{
    ElementFixedSize, ElementFixedWidthGrowingHeight, position::Position,
    traits::ElementFixedSizeTrait,
};

/// Place elements next to each other in rows.
/// If the next element wouldn't fit within wrap_width, place it on a new row.
/// Adds h_spacing between elements, and v_spacing between rows.
/// Allows element's height to grow to match the talles element in their row.
/// Width of this container is the width of the widest row - this might be smaller than wrap_width
pub struct HorizontalWrappingContainer<'a, BackendContext, UserState> {
    rows_max_height: Vec<usize>,
    last_row_width: usize,
    max_row_width: usize,
    render: Box<dyn 'a + FnOnce(&mut BackendContext, Position, usize) -> UserState>,

    h_spacing: usize,
    v_spacing: usize,
    wrap_width: usize,
}

impl<'a, BackendContext: 'a> HorizontalWrappingContainer<'a, BackendContext, ()> {
    pub fn new(h_spacing: usize, v_spacing: usize, wrap_width: usize) -> Self {
        Self {
            rows_max_height: vec![0],
            last_row_width: 0,
            max_row_width: 0,
            render: Box::new(|_, _, _| ()),

            h_spacing,
            v_spacing,
            wrap_width,
        }
    }
}

impl<'a, BackendContext: 'a, PrevUserState: 'a>
    HorizontalWrappingContainer<'a, BackendContext, PrevUserState>
{
    pub fn add_child<
        T: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, ChildUserState>>,
        ChildUserState: 'a,
        F: 'a + FnOnce(PrevUserState, ChildUserState) -> UserState,
        UserState,
    >(
        mut self,
        child: T,
        state_closure: F,
    ) -> Result<HorizontalWrappingContainer<'a, BackendContext, UserState>, ()> {
        let child = child.into();
        if child.width() > self.wrap_width {
            return Err(());
        }

        let child_height = child.min_height();

        let new_closure: Box<dyn 'a + FnOnce(&mut BackendContext, Position, usize) -> UserState> =
            if self.last_row_width + self.h_spacing + child.width() > self.wrap_width {
                // would overflow: create a new row
                let closure_max_height = *self.rows_max_height.last().unwrap();
                self.last_row_width = child.width();
                let pos = Position::new(
                    0,
                    self.rows_max_height
                        .iter()
                        .map(|h| h + self.v_spacing)
                        .sum::<usize>(),
                );
                self.rows_max_height.push(0);
                let closure = move |ctx: &mut BackendContext, top_left, max_height| {
                    let prev_rows_state = (self.render)(ctx, top_left, closure_max_height);
                    let child_state = child.render(ctx, top_left + pos, max_height);
                    (state_closure)(prev_rows_state, child_state)
                };
                Box::new(closure)
            } else {
                // stay on the same row
                let pos = Position::new(
                    self.last_row_width
                        + if self.max_row_width == 0 {
                            // don't add spacing before the first element
                            0
                        } else {
                            self.h_spacing
                        },
                    self.rows_max_height
                        .iter()
                        .map(|h| h + self.v_spacing)
                        .sum::<usize>()
                        - self.rows_max_height.last().unwrap_or(&0)
                        - self.v_spacing,
                );
                self.last_row_width += self.h_spacing + child.width();

                let closure = move |ctx: &mut BackendContext, top_left, max_height| {
                    let prev_state = (self.render)(ctx, top_left, max_height);
                    let child_state = child.render(ctx, top_left + pos, max_height);
                    (state_closure)(prev_state, child_state)
                };

                Box::new(closure)
            };

        let last = self.rows_max_height.last_mut().unwrap();
        *last = (*last).max(child_height);
        self.max_row_width = self.max_row_width.max(self.last_row_width);

        Ok(HorizontalWrappingContainer {
            rows_max_height: self.rows_max_height,
            last_row_width: self.last_row_width,
            max_row_width: self.max_row_width,
            render: new_closure,
            h_spacing: self.h_spacing,
            v_spacing: self.v_spacing,
            wrap_width: self.wrap_width,
        })
    }
}

impl<'a, BackendContext: 'a, UserState: 'a> ElementFixedSizeTrait<'a, BackendContext, UserState>
    for HorizontalWrappingContainer<'a, BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.max_row_width
    }

    fn height(&self) -> usize {
        let num_rows = self.rows_max_height.len();
        let spacing = if num_rows == 0 { 0 } else { num_rows - 1 };
        self.rows_max_height.iter().sum::<usize>() + spacing
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: crate::position::Position,
    ) -> UserState {
        (self.render)(ctx, top_left, *self.rows_max_height.last().unwrap())
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
    for HorizontalWrappingContainer<'a, BackendContext, UserState>
{
    fn into(self) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(self),
        }
    }
}
