use angui::{
    backends::{
        self,
        print_backend::{CharRectangle, PrintBackendCTX},
    },
    containers::{border_container::BorderContainer, horizontal_container::HorizontalContainer, padding_container::PaddingContainer},
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
    widgets::{label::Label, rectangle::RectangleElement},
};

trait SizedPrint: Render<PrintBackendCTX> + FixedWidth + FixedHeight {}
impl<T> SizedPrint for T where T: Render<PrintBackendCTX> + FixedWidth + FixedHeight {}

fn main() {
    let mut ctx = backends::print_backend::PrintBackendCTX::new(150, 50);

    let root = BorderContainer::new(Box::new(HorizontalContainer::new()
        .add_child(CharRectangle::new(30, 10, 'a') as Box<dyn SizedPrint>)
        .add_child(PaddingContainer::all(Label::new("Test label"), 1))
        .add_child(CharRectangle::new(5, 15, 'b'))
        .add_child(RectangleElement::new(2, 2))));

    root.render(&mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
