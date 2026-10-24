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
            character: '█',
            top: Coordinate::new(column, 0),
            length,
        }
    }
    pub fn move_up<T: Terminal>(&mut self, term: &mut T) -> Result<(), TerminalError> {
        self.clear(term)?;
        self.top.y = self.top.y.saturating_sub(1);
        Ok(())
    }
    pub fn move_down<T: Terminal>(&mut self, term: &mut T) -> Result<(), TerminalError> {
        self.clear(term)?;
        let max_y = term.size().y;
        self.top.y = (self.top.y + 1).min(max_y - self.length);
        Ok(())
    }
}

impl<T: Terminal> Drawable<T> for Paddle {
    fn clear(&self, term: &mut T) -> Result<(), TerminalError> {
        for i in 0..self.length {
            term.erase(&Coordinate::new(self.top.x, self.top.y + i))?;
        }
        Ok(())
    }
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        for i in 0..self.length {
            term.draw(&Coordinate::new(self.top.x, self.top.y + i), self.character)?;
        }
        Ok(())
    }
}
