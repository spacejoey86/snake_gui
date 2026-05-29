use angui::{
    backends::{self, print_backend::CharRectangle},
    containers::{
        border_container::BorderContainer, horizontal_container::HorizontalContainer,
        padding_container::PaddingContainer,
    },
    position::Position,
    traits::{FixedHeight, FixedWidth, Render},
    widgets::{label::Label, separator::VerticalSeparator, spacer::HorizontalSpacer},
};

fn main() {
    let root = BorderContainer::new(
        HorizontalContainer::new(1)
            .add_child(CharRectangle::new(30, 10, 'a'))
            .add_child(VerticalSeparator::new())
            .add_child(PaddingContainer::all(Label::new("Test label"), 1))
            .add_child(CharRectangle::new(5, 15, 'b'))
            .add_child(HorizontalSpacer::new(1))
            .add_child(CharRectangle::new(3, 3, 'c')),
    );

    let mut ctx = backends::print_backend::PrintBackendCTX::new(root.width(), root.height()); // create a buffer that will fit the contents
    Render::render(&root, &mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
