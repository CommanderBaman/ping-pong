use crate::{
    drawable::Drawable,
    model::{Coordinate, Point},
    terminal::{Terminal, TerminalError},
};

pub struct Paddle {
    character: char,
    top: Coordinate,
    length: Point,
}

impl Paddle {
    pub fn new(length: Point, column: Point) -> Self {
        assert!(length % 2 == 1);
        Self {
            character: '|',
            top: Coordinate::new(column, 0),
            length,
        }
    }
}

impl<T: Terminal> Drawable<T> for Paddle {
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        for i in 0..self.length {
            term.draw(&Coordinate::new(self.top.x, self.top.y + i), self.character)?;
        }
        Ok(())
    }
}
