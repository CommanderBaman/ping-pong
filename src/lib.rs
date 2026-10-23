mod drawable;
mod model;
mod terminal;

use std::{io, time::Duration};

use crossterm::event::{Event, poll, read};

use crate::{
    drawable::{Ball, Drawable, Paddle},
    model::Coordinate,
    terminal::{Terminal, TerminalImpl},
};

fn pause() -> Result<(), io::Error> {
    loop {
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => {
                    if event.code.is_char('c') {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn run() {
    let Ok(mut term) = TerminalImpl::new() else {
        return;
    };
    let paddle_left = Paddle::new(5, 0);
    let paddle_right = Paddle::new(5, 520);
    let mut ball = Ball::new();

    let _ = term.clear();
    paddle_left.draw(&mut term).unwrap();
    paddle_right.draw(&mut term).unwrap();
    ball.place(Coordinate::new(1, 2));
    ball.draw(&mut term).unwrap();
    let _ = pause();
}
