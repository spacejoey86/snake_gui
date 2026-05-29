use crate::position::Position;

pub trait UnsizedElement {}

/// a ui element that can calculate it's X size independently of parents
/// will need to be generic over the backend so that backends can implement it on foreign types?
pub trait FixedWidth {
    fn width(&self) -> usize;
}
/// a ui element that can calculate it's Y size independently of parents
pub trait FixedHeight {
    fn height(&self) -> usize;
}

/// like egui Widget trait
/// given sizes (?) render yourself?
/// for a GUI on a screen, the top level has a size
/// but some things might not - windows which can resize, document of arbitrary size
/// does this actually render, or just layout
/// depends on your backend?
pub trait Render<BackendContext> {
    fn render(&self, ctx: &mut BackendContext, top_left: Position);
}
