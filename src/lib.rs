mod drawable;
mod model;
mod screen;

use std::io::{self, Read, Write};

use crate::{
    drawable::{Ball, Drawable, Paddle},
    model::Coordinate,
    screen::{Screen, TmuxScreen},
};

fn pause() {
    let mut stdout = io::stdout();
    stdout.flush().unwrap();

    let mut buffer = [0; 1];
    let _ = io::stdin().read(&mut buffer);
}

pub fn run() {
    let screen = TmuxScreen::new();
    let paddle_left = Paddle::new(5, 0);
    let paddle_right = Paddle::new(5, 120);
    let mut ball = Ball::new();

    screen.clear();
    paddle_left.draw(&screen).unwrap();
    paddle_right.draw(&screen).unwrap();
    ball.place(Coordinate::new(1, 2));
    ball.draw(&screen).unwrap();
    pause();
}
