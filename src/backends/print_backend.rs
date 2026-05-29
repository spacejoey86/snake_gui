use crate::{
    layout_traits::{FixedHeight, FixedWidth, Render},
    widgets::{label::Label, rectangle::RectangleElement},
};

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
            println!(
                "{}",
                line.iter()
                    .fold("".to_string(), |l, r| format!("{}{}", l, r).to_string())
            )
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

pub struct CharRectangle {
    width: usize,
    height: usize,
    char: char,
}

impl CharRectangle {
    pub fn new(width: usize, height: usize, char: char) -> Self {
        Self {
            width,
            height,
            char,
        }
    }
}

impl Render<PrintBackendCTX> for CharRectangle {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: crate::position::Position) {
        for y in 0..self.height {
            for x in 0..self.width {
                ctx.buffer[top_left.y + y][top_left.x + x] = self.char;
            }
        }
    }
}

impl FixedWidth for CharRectangle {
    fn get_x_size(&self) -> usize {
        self.width
    }
}

impl FixedHeight for CharRectangle {
    fn get_y_size(&self) -> usize {
        self.height
    }
}

impl FixedHeight for Label {
    fn get_y_size(&self) -> usize {
        1
    }
}

impl FixedWidth for Label {
    fn get_x_size(&self) -> usize {
        self.text.len()
    }
}

impl Render<PrintBackendCTX> for Label {
    fn render(&self, ctx: &mut PrintBackendCTX, top_left: crate::position::Position) {
        for (i, char) in self.text.chars().enumerate() {
            ctx.buffer[top_left.y][top_left.x + i] = char
        }
    }
}
