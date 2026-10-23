mod ball;
mod paddle;

use crate::terminal::{Terminal, TerminalError};

pub trait Drawable<T: Terminal> {
    fn draw(&self, term: &mut T) -> Result<(), TerminalError>;
}

pub type Ball = ball::Ball;
pub type Paddle = paddle::Paddle;
