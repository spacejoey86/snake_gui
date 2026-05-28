use crate::{layout_traits::Render, widgets::rectangle::RectangleElement};

pub struct PrintBackendCTX {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Vec<char>>,
}

impl PrintBackendCTX {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![' '; width]; height],
        }
    }

    pub fn display(self) {
        for line in self.buffer {
            println!("{}", line.iter().fold("".to_string(), |l, r| format!("{}{}", l, r).to_string()))
        }
    }
}

impl Render<PrintBackendCTX> for RectangleElement {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: crate::position::Position) {
        for row in 0..self.height {
            for column in 0..self.width {
                ctx.buffer[top_left.y + row][top_left.x + column] = '█';
            }
        }
    }
}
