mod visual_containers;
mod widgets;
pub use widgets::char_rectangle::CharRectangle;

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
