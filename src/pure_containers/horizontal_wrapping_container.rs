use crate::{
    position::Position,
    pure_containers::horizontal_container::ContainerElement,
    traits::{FixedHeight, FixedWidth, GrowingHeight, Render, RenderGrowHeight},
};

pub struct HorizontalWrappingContainer<T: ?Sized> {
    children: Vec<Vec<Box<T>>>,
    h_spacing: usize,
    v_spacing: usize,
    wrap_width: usize,
}

impl<BackendContext> FixedWidth
    for HorizontalWrappingContainer<dyn ContainerElement<BackendContext>>
{
    fn width(&self) -> usize {
        self.children
            .iter()
            .map(|row| self.row_width(row))
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized> FixedHeight for HorizontalWrappingContainer<T>
where
    T: GrowingHeight,
{
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
}

impl<T: ?Sized> HorizontalWrappingContainer<T> {
    pub fn new(h_spacing: usize, v_spacing: usize, wrap_width: usize) -> Box<Self> {
        Box::new(Self {
            children: vec![vec![]],
            h_spacing,
            v_spacing,
            wrap_width,
        })
    }
}

impl<BackendContext> HorizontalWrappingContainer<dyn ContainerElement<BackendContext>> {
    pub fn add_child(
        mut self,
        child: Box<dyn ContainerElement<BackendContext>>,
    ) -> Result<Box<Self>, ()> {
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

    fn row_width(&self, row: &Vec<Box<dyn ContainerElement<BackendContext>>>) -> usize {
        let spacing = if row.len() == 0 {
            0
        } else {
            self.h_spacing * (row.len() - 1)
        };
        spacing + row.iter().map(|child| child.width()).sum::<usize>()
    }
}

impl<T: ?Sized> HorizontalWrappingContainer<T>
where
    T: GrowingHeight,
{
    fn row_height(row: &Vec<Box<T>>) -> usize {
        row.iter()
            .map(|child| child.min_height())
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized, BackendContext> Render<BackendContext> for HorizontalWrappingContainer<T>
where
    T: FixedWidth + GrowingHeight + RenderGrowHeight<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: crate::position::Position) {
        let mut y_offset = 0;
        for row in &self.children {
            let mut x_offset = 0;
            let height = HorizontalWrappingContainer::row_height(row);
            for child in row {
                child.render(ctx, top_left + Position::new(x_offset, y_offset), height);
                x_offset += child.width() + self.h_spacing;
            }
            y_offset += height + self.v_spacing
        }
    }
}
