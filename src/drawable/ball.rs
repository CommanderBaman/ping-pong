use crate::{
    config::Config,
    drawable::Drawable,
    model::{Coordinate, Position, Velocity},
    random,
    terminal::{Edge, Terminal, TerminalError},
};

pub struct Ball {
    character: char,
    location: Position,
    velocity: Velocity,
    random_bounce: bool,
}

impl Ball {
    pub fn new(config: &Config) -> Self {
        Self {
            character: '⏺',
            location: Position::new(0.0, 0.0),
            velocity: Velocity::new(1.0, 1.0).norm().power(config.ball_speed),
            random_bounce: config.random_bounce,
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
    pub fn bounce_horizontal(&mut self) {
        self.velocity.x = -1.0 * self.velocity.x;
        if self.random_bounce {
            let speed = self.velocity.magnitude();
            self.velocity.x = random::f32();
            self.velocity.y = random::f32();
            self.velocity = self.velocity.norm().power(speed);
        }
    }
}

impl<T: Terminal> Drawable<T> for Ball {
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        term.draw(&self.coordinate(), self.character)
    }
    fn clear(&self, term: &mut T) -> Result<(), TerminalError> {
        term.erase(&self.coordinate())
    }
}
