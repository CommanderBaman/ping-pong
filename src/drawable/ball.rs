use crate::{
    config::Config,
    drawable::Drawable,
    model::{Coordinate, Position, Real, Velocity},
    random,
    terminal::{Edge, Terminal, TerminalError},
};

pub struct Ball {
    character: char,
    location: Position,
    velocity: Velocity,
    random_bounce: bool,
    speed_multiplier: Real,
    dt: Real,
}

impl Ball {
    pub fn new(config: &Config) -> Self {
        Self {
            character: '⏺',
            location: Position::new(0.0, 0.0),
            velocity: Velocity::new(1.0, 1.0)
                .norm()
                .power(config.initial_ball_speed),
            random_bounce: config.random_bounce,
            speed_multiplier: config.ball_speed_multiplier,
            // 1 s = 1 step, dt will take frames per second as input
            dt: 1.0 / config.frames_per_second as Real,
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
        self.location += self.velocity * self.dt;
        Ok(())
    }
    fn speed(&mut self) -> Real {
        self.velocity.magnitude()
    }
    pub fn bounce_horizontal(&mut self) {
        let speed = self.speed();
        self.velocity = self.velocity.norm().power(speed * self.speed_multiplier);
        self.velocity.x = -1.0 * self.velocity.x;
        if self.random_bounce {
            let speed = self.speed();
            self.velocity.x = random::real();
            self.velocity.y = random::real();
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
