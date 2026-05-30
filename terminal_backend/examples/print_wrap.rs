use terminal_backend::{CharRectangle, PrintBackendCTX};

use angui::{
    
    position::Position,
    pure_containers::{HorizontalWrappingContainer, PaddingContainer},
    spacers::HorizontalSpacer,
    visual_containers::BorderContainer,
    widgets::{Label, VerticalSeparator},
    {FixedHeight, FixedWidth, Render},
};

fn main() {
    let root = BorderContainer::new(
        HorizontalWrappingContainer::new(1, 1, 50)
            .add_child(CharRectangle::new(30, 10, 'a'))
            .unwrap()
            .add_child(VerticalSeparator::new())
            .unwrap()
            .add_child(PaddingContainer::all(Label::new("Test label"), 1))
            .unwrap()
            .add_child(CharRectangle::new(5, 15, 'b'))
            .unwrap()
            .add_child(HorizontalSpacer::new(1))
            .unwrap()
            .add_child(CharRectangle::new(3, 3, 'c'))
            .unwrap(),
    );

    let mut ctx = PrintBackendCTX::new(root.width(), root.height()); // create a buffer that will fit the contents
    Render::render(&root, &mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
