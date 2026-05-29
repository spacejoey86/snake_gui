use angui::{
    backends::{
        self,
        print_backend::{CharRectangle, PrintBackendCTX},
    },
    containers::horizontal_container::HorizontalContainer,
    traits::{FixedHeight, FixedWidth, Render},
    position::Position,
    widgets::{label::Label, rectangle::RectangleElement},
};

trait SizedPrint: Render<PrintBackendCTX> + FixedWidth + FixedHeight {}
impl<T> SizedPrint for T where T: Render<PrintBackendCTX> + FixedWidth + FixedHeight {}

fn main() {
    let mut ctx = backends::print_backend::PrintBackendCTX::new(150, 50);

    let root = HorizontalContainer::new()
        .add_child(Box::new(CharRectangle::new(30, 10, 'a')) as Box<dyn SizedPrint>)
        .add_child(Box::new(Label::new("Test label")))
        .add_child(Box::new(CharRectangle::new(5, 15, 'b')))
        .add_child(Box::new(RectangleElement::new(2, 2)));

    root.render(&mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
