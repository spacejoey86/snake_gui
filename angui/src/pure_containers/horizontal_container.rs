use crate::{ElementFixedSize, ElementFixedWidthGrowingHeight, position::Position, traits::ElementFixedSizeTrait};

/// Place elements one after the other horizontally.
/// Adds spacing between elements.
/// Allows element's height to grow to match the tallest element.
pub struct HorizontalContainer<BackendContext> {
    children: Vec<ElementFixedWidthGrowingHeight<BackendContext>>,
    spacing: usize,
}

impl<BackendContext> ElementFixedSizeTrait<BackendContext> for HorizontalContainer<BackendContext> {
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

    fn height(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.min_height())
            .max()
            .unwrap_or(0)
    }

    fn render(&self, ctx: &mut BackendContext, top_left: Position) {
        let mut x_offset = 0;
        let height = self.height();
        for child in self.children.iter() {
            child.render(ctx, top_left + Position::new(x_offset, 0), height);
            x_offset += child.width() + self.spacing
        }
    }
}

impl<BackendContext: 'static> HorizontalContainer<BackendContext> {
    pub fn add_child(mut self, child: ElementFixedWidthGrowingHeight<BackendContext>) -> Box<Self> {
        self.children.push(child);
        return Box::new(self);
    }

    pub fn new(spacing: usize) -> Self {
        Self {
            children: vec![],
            spacing,
        }
    }

    pub fn build(self) -> ElementFixedSize<BackendContext> {
        ElementFixedSize { inner: Box::new(self) }
    }
}
