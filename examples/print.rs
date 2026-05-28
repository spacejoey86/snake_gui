use angui::{
    backends::{self, print_backend::CharRectangle}, containers::horizontal_container::HorizontalContainer, layout_traits::Render,
    position::Position, widgets::rectangle::RectangleElement,
};

fn main() {
    let mut ctx = backends::print_backend::PrintBackendCTX::new(150, 50);

    let root = HorizontalContainer::new()
        .add_child(CharRectangle::new(30, 10, 'a'))
        .add_child(CharRectangle::new(5, 15, 'b'));

    root.render(&mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
