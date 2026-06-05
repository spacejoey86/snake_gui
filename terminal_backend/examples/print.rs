use terminal_backend::{CharRectangle, PrintBackendCTX};

use angui::{
    Position,
    pure_containers::{HorizontalContainer, PaddingContainer},
    spacers::HorizontalSpacer,
    visual_containers::BorderContainer,
    widgets::{Label, VerticalSeparator},
};

fn main() {
    let root = BorderContainer::new(
        HorizontalContainer::new(1)
            .add_child(CharRectangle::new(30, 10, 'a'))
            .add_child(VerticalSeparator::new())
            .add_child(PaddingContainer::all(Label::new("Test label"), 1))
            .add_child(CharRectangle::new(5, 15, 'b'))
            .add_child(HorizontalSpacer::new(1))
            .add_child(CharRectangle::new(3, 3, 'c'))
            .build(),
    );

    let mut ctx = PrintBackendCTX::new(root.width(), root.height()); // create a buffer that will fit the contents
    root.render(&mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
