use crate::{
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
};

/// put elements one after the other horizontally
/// elements must have a known size
/// if your elements don't have a known size, consider using (TODO)
/// probably needs to box the elements? or can I use references somehow
pub struct HorizontalContainer<T: ?Sized> {
    children: Vec<Box<T>>,
}

impl<T: ?Sized> FixedWidth for HorizontalContainer<T>
where
    T: FixedWidth,
{
    fn width(&self) -> usize {
        self.children.iter().map(|child| child.width()).sum()
    }
}

impl<T: ?Sized> FixedHeight for HorizontalContainer<T>
where
    T: FixedHeight,
{
    fn height(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.height())
            .max()
            .unwrap_or(0)
    }
}

impl<T: ?Sized, BackendContext> Render<BackendContext> for HorizontalContainer<T>
where
    T: FixedWidth + Render<BackendContext>,
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        let mut x_offset = 0;
        for child in self.children.iter() {
            child.render(ctx, top_left + Position::new(x_offset, 0));
            x_offset += child.width()
        }
    }
}

impl<T: ?Sized> HorizontalContainer<T> {
    pub fn add_child(mut self, child: Box<T>) -> Self {
        self.children.push(child);
        return self;
    }

    pub fn new() -> Box<Self> {
        Box::new(Self { children: vec![] })
    }
}
