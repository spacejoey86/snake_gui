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
        let spacing = if self.children.len() == 1 {
            0
        } else {
            self.spacing * self.children.len()
        };
        spacing
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

/// trait to help rust infer types
pub trait ContainerElement<BackendContext>: RenderGrowHeight<BackendContext> + FixedWidth + GrowingHeight {}
impl<T, BackendContext> ContainerElement<BackendContext> for T where T: RenderGrowHeight<BackendContext> + FixedWidth + GrowingHeight {}

impl<BackendContext> HorizontalContainer<dyn ContainerElement<BackendContext>> {
    pub fn add_child(mut self, child: Box<dyn ContainerElement<BackendContext>>) -> Box<Self> {
        self.children.push(child);
        return Box::new(self);
    }

    pub fn new(spacing: usize) -> Box<Self> {
        Box::new(Self {
            children: vec![],
            spacing,
        })
    }
}
