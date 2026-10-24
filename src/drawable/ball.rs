use crate::{
    constants,
    drawable::Drawable,
    model::{Coordinate, Position, Velocity},
    terminal::{Edge, Terminal, TerminalError},
};

pub struct Ball {
    character: char,
    location: Position,
    velocity: Velocity,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            character: 'O',
            location: Position::new(0.0, 0.0),
            velocity: Velocity::new(1.0, 1.0),
        }
    }
    pub fn coordinate(&self) -> Coordinate {
        // TODO: see what happens here
        // copy or clone
        self.location.into()
    }
    pub fn place(&mut self, coordinate: Coordinate) {
        self.location = coordinate.into();
    }
    pub fn update<T: Terminal>(&mut self, term: &mut T) -> Result<(), TerminalError> {
        self.clear(term)?;
        match term.edge(&self.coordinate()) {
            Some(Edge::TOP) => self.velocity.y = self.velocity.y.abs(),
            Some(Edge::BOTTOM) => self.velocity.y = -1.0 * self.velocity.y.abs(),
            Some(Edge::LEFT) => self.velocity.x = self.velocity.x.abs(),
            Some(Edge::RIGHT) => self.velocity.x = -1.0 * self.velocity.x.abs(),
            None => {}
        }
        self.location += self.velocity;
        Ok(())
    }
}

impl<T: Terminal> Drawable<T> for Ball {
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        term.draw(&self.coordinate(), self.character)
    }
    fn clear(&self, term: &mut T) -> Result<(), TerminalError> {
        term.draw(&self.coordinate(), constants::EMPTY_CHARACTER)
    }
}
