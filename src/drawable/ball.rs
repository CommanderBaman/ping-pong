use crate::{
    drawable::Drawable,
    model::{Coordinate, Velocity},
    terminal::{Terminal, TerminalError},
};

pub struct Ball {
    character: char,
    location: Coordinate,
    velocity: Velocity,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            character: 'O',
            location: Coordinate::new(0, 0),
            velocity: Velocity::new(0, 0),
        }
    }
    pub fn place(&mut self, coordinate: Coordinate) {
        self.location = coordinate;
    }
}

impl<T: Terminal> Drawable<T> for Ball {
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        term.draw(&self.location, self.character)
    }
}
