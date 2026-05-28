use crate::{layout_traits::{KnownXSizeElement, KnownYSizeElement, Render}, position::Position};

/// put elements one after the other horizontally
/// elements must have a known size
/// if your elements don't have a known size, consider using (TODO)
/// probably needs to box the elements? or can I use references somehow
pub struct HorizontalContainer<T> {
    children: Vec<T>,
}

impl<T> HorizontalContainer<T> {
    pub fn add_child(mut self, child: T) -> Self {
        self.children.push(child);
        return self;
    }
}

impl<T> KnownXSizeElement for HorizontalContainer<T>
where
    T: KnownXSizeElement,
{
    fn get_x_size(&self) -> usize {
        self.children.iter().map(|child| child.get_x_size()).sum()
    }
}

impl<T> KnownYSizeElement for HorizontalContainer<T>
where
    T: KnownYSizeElement,
{
    fn get_y_size(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.get_y_size())
            .max()
            .unwrap_or(0)
    }
}

impl<T, BackendContext> Render<BackendContext> for HorizontalContainer<T>
where
    T : KnownXSizeElement + Render<BackendContext>
{
    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        let mut x_offset = 0;
        for child in self.children.iter() {
            child.render(ctx, top_left + Position::new(x_offset, 0));
            x_offset += child.get_x_size()
        }
    }
}
