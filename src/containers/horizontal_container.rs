use crate::{
    position::Position,
    traits::{FixedHeight, FixedWidth, GrowingHeight, Render, RenderGrowHeight},
};

/// put elements one after the other horizontally
/// elements must have a known size
/// if your elements don't have a known size, consider using (TODO)
/// probably needs to box the elements? or can I use references somehow
pub struct HorizontalContainer<T: ?Sized> {
    children: Vec<Box<T>>,
    spacing: usize,
}

impl<T: ?Sized> FixedWidth for HorizontalContainer<T>
where
    T: FixedWidth,
{
    fn width(&self) -> usize {
        self.spacing * self.children.len()
            + self
                .children
                .iter()
                .map(|child| child.width())
                .sum::<usize>()
    }
}

impl<T: ?Sized> FixedHeight for HorizontalContainer<T>
where
    T: GrowingHeight,
{
    fn height(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.min_height())
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized, BackendContext> Render<BackendContext> for HorizontalContainer<T>
where
    T: FixedWidth + RenderGrowHeight<BackendContext> + GrowingHeight,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        let mut x_offset = 0;
        let height = self.height();
        for child in self.children.iter() {
            child.render(ctx, top_left + Position::new(x_offset, 0), height);
            x_offset += child.width() + self.spacing
        }
    }
}

impl<T: ?Sized> HorizontalContainer<T> {
    pub fn add_child(mut self, child: Box<T>) -> Self {
        self.children.push(child);
        return self;
    }

    pub fn new(spacing: usize) -> Box<Self> {
        Box::new(Self {
            children: vec![],
            spacing,
        })
    }
}
