use angui::{
    backends::{
        self,
        print_backend::{CharRectangle, PrintBackendCTX},
    },
    containers::{
        border_container::BorderContainer, horizontal_wrapping_container::HorizontalWrappingContainer, padding_container::PaddingContainer
    },
    position::Position,
    traits::{FixedHeight, FixedWidth, GrowingHeight, Render, RenderGrowHeight},
    widgets::{label::Label, separator::VerticalSeparator, spacer::HorizontalSpacer},
};

trait SizedPrint: RenderGrowHeight<PrintBackendCTX> + FixedWidth + GrowingHeight {}
impl<T> SizedPrint for T where T: RenderGrowHeight<PrintBackendCTX> + FixedWidth + GrowingHeight {}

fn main() {
    let root = Box::leak(BorderContainer::new(Box::new(
        HorizontalWrappingContainer::new(1, 1, 50)
            .add_child(CharRectangle::new(30, 10, 'a') as Box<dyn SizedPrint>).unwrap()
            .add_child(VerticalSeparator::new()).unwrap()
            .add_child(PaddingContainer::all(Label::new("Test label"), 1)).unwrap()
            .add_child(CharRectangle::new(5, 15, 'b')).unwrap()
            .add_child(HorizontalSpacer::new(1)).unwrap()
            .add_child(CharRectangle::new(3, 3, 'c')).unwrap()
    )));

    let mut ctx = backends::print_backend::PrintBackendCTX::new(root.width(), root.height()); // create a buffer that will fit the contents
    Render::render(root, &mut ctx, Position::new(0, 0)); // render onto buffer
    ctx.display(); // print the buffer to the terminal
}
