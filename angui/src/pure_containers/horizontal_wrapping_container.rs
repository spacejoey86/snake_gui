use std::marker::PhantomData;

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
    children: Vec<Vec<ElementFixedWidthGrowingHeight<'a, BackendContext, UserState>>>,
    h_spacing: usize,
    v_spacing: usize,
    wrap_width: usize,
    phantom: PhantomData<BackendContext>,
}

impl<'a, BackendContext: 'static, UserState: 'static>
    HorizontalWrappingContainer<'a, BackendContext, UserState>
{
    pub fn new(h_spacing: usize, v_spacing: usize, wrap_width: usize) -> Self {
        Self {
            children: vec![vec![]],
            h_spacing,
            v_spacing,
            wrap_width,
            phantom: PhantomData,
        }
    }

    // todo: replace with into
    pub fn build(self) -> ElementFixedSize<'a, BackendContext, UserState> {
        ElementFixedSize {
            inner: Box::new(self).covariant_box(),
        }
    }

    pub fn add_child<T: Into<ElementFixedWidthGrowingHeight<'a, BackendContext, UserState>>>(
        mut self,
        child: T,
    ) -> Result<Box<Self>, ()> {
        let child = child.into();
        if child.width() > self.wrap_width {
            return Err(());
        }

        let last_row = self.children.last().unwrap();
        let last_row_width = self.row_width(last_row);
        if last_row_width + child.width() + last_row_width.min(1) > self.wrap_width {
            // would overflow: create a new row
            self.children.push(vec![])
        }
        self.children.last_mut().unwrap().push(child);

        return Ok(Box::new(self));
    }

    fn row_width(
        &self,
        row: &Vec<ElementFixedWidthGrowingHeight<BackendContext, UserState>>,
    ) -> usize {
        let spacing = if row.len() == 0 {
            0
        } else {
            self.h_spacing * (row.len() - 1)
        };
        spacing + row.iter().map(|child| child.width()).sum::<usize>()
    }

    fn row_height(row: &Vec<ElementFixedWidthGrowingHeight<BackendContext, UserState>>) -> usize {
        row.iter()
            .map(|child| child.min_height())
            .max()
            .unwrap_or(0)
    }

    fn covariant<'b>(self: Box<Self>) -> HorizontalWrappingContainer<'b, BackendContext, UserState>
    where
        'a: 'b,
    {
        HorizontalWrappingContainer {
            children: self
                .children
                .into_iter()
                .map(|row| row.into_iter().map(|child| child.covariant()).collect())
                .collect(),
            h_spacing: self.h_spacing,
            v_spacing: self.v_spacing,
            wrap_width: self.wrap_width,
            phantom: PhantomData,
        }
    }
}

impl<'a, BackendContext: 'static, UserState: 'static>
    ElementFixedSizeTrait<'a, BackendContext, UserState>
    for HorizontalWrappingContainer<'a, BackendContext, UserState>
{
    fn width(&self) -> usize {
        self.children
            .iter()
            .map(|row| self.row_width(row))
            .max()
            .unwrap_or(0)
    }

    fn height(&self) -> usize {
        let spacing = if self.children.len() == 0 {
            0
        } else {
            (self.children.len() - 1) * self.v_spacing
        };

        self.children
            .iter()
            .map(|row| {
                row.iter()
                    .map(|row_child| row_child.min_height())
                    .max()
                    .unwrap_or(0)
            })
            .sum::<usize>()
            + spacing
    }

    fn render(
        self: Box<Self>,
        ctx: &mut BackendContext,
        top_left: crate::position::Position,
    ) -> UserState {
        let mut y_offset = 0;
        for row in self.children {
            let mut x_offset = 0;
            let height = HorizontalWrappingContainer::row_height(&row);
            for child in row {
                let width = child.width();
                child.render(ctx, top_left + Position::new(x_offset, y_offset), height);
                x_offset += width + self.h_spacing;
            }
            y_offset += height + self.v_spacing
        }

        todo!()
    }

    fn covariant_box<'b>(
        self: Box<Self>,
    ) -> Box<dyn ElementFixedSizeTrait<'b, BackendContext, UserState> + 'b>
    where
        'a: 'b,
    {
        Box::new(self.covariant())
    }
}
