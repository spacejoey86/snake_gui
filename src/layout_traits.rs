use crate::position::Position;

pub trait UnsizedElement {}

/// a ui element that can calculate it's X size independently of parents
pub trait KnownXSizeElement {
    fn get_x_size(&self) -> usize;
}
/// a ui element that can calculate it's Y size independently of parents
pub trait KnownYSizeElement {
    fn get_y_size(&self) -> usize;
}

/// knows both x and y sizes
/// does this need to be its own trait?
/// might be convenient for writing signatures?
pub trait SizedElement : KnownXSizeElement + KnownYSizeElement {

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
