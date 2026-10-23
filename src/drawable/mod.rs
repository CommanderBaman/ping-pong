mod ball;
mod paddle;

use crate::screen::{OutOfBoundError, Screen};

pub trait Drawable<T: Screen> {
    fn draw(&self, screen: &T) -> Result<(), OutOfBoundError>;
}

pub type Ball = ball::Ball;
pub type Paddle = paddle::Paddle;
