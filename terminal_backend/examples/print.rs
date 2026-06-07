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
            .add_child(CharRectangle::new(30, 10, 'a'), |_, _| ())
            .add_child(VerticalSeparator::new(), |_, _| ())
            .add_child(
                PaddingContainer::all(Label::new("Test label"), 1),
                |_, _| (),
            )
            .add_child(CharRectangle::new(5, 15, 'b'), |_, _| ())
            .add_child(VerticalSeparator::new(), |_, _| ())
            .add_child(HorizontalSpacer::new(1), |_, _| ())
            .add_child(CharRectangle::new(3, 3, 'c'), |_, _| ()),
    );

    let mut ctx = PrintBackendCTX::new(root.width(), root.height()); // create a buffer that will fit the contents
    root.render(&mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
