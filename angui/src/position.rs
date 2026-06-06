use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// if self is the top left of a rectangle, could other be in the box
    pub fn inside_top_left(&self, other: Position) -> bool {
        self.x < other.x && self.y < other.y
    }

    /// if self is the bottom left of a rectangle, could other be in the box
    pub fn inside_bottom_right(&self, other: Position) -> bool {
        self.x > other.x && self.y > other.y
    }
}

impl Add for Position {
    type Output = Position;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
