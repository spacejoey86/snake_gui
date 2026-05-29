use crate::{
    layout_traits::{FixedWidth, FixedHeight, Render},
    position::Position,
};

/// put elements one after the other horizontally
/// elements must have a known size
/// if your elements don't have a known size, consider using (TODO)
/// probably needs to box the elements? or can I use references somehow
pub struct HorizontalContainer<T: ?Sized> {
    children: Vec<Box<T>>,
}

impl<T> FixedWidth for HorizontalContainer<T>
where
    T: FixedWidth,
{
    fn get_x_size(&self) -> usize {
        self.children.iter().map(|child| child.get_x_size()).sum()
    }
}

impl<T> FixedHeight for HorizontalContainer<T>
where
    T: FixedHeight,
{
    fn get_y_size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.get_y_size())
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
            x_offset += child.get_x_size()
        }
    }
}

impl<T: ?Sized> HorizontalContainer<T> {
    pub fn add_child(mut self, child: Box<T>) -> Self {
        self.children.push(child);
        return self;
    }

    pub fn new() -> Self {
        Self { children: vec![] }
    }
}
