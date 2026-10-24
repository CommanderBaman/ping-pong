mod constants;
mod drawable;
mod model;
mod screen;
mod terminal;

use std::time::Duration;

use crossterm::event::{Event, poll, read};
use thiserror::Error;

use crate::{
    drawable::{Ball, Drawable, Paddle},
    terminal::{Terminal, TerminalError, TerminalImpl},
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Terminal(#[from] TerminalError),
}

fn game_loop<T: Terminal>(mut term: T) -> Result<(), AppError> {
    let term_size = term.size();
    let mut paddle_left = Paddle::new(constants::PADDLE_LENGTH, 0);
    let mut paddle_right = Paddle::new(constants::PADDLE_LENGTH, term_size.x);
    let mut ball = Ball::new();
    let frame_time = Duration::from_millis(1000 / constants::FRAMES_PER_SECOND);
    loop {
        if poll(frame_time)? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('k') => paddle_left.move_up(&mut term)?,
                    Some('j') => paddle_left.move_down(&mut term)?,
                    Some('c') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // update the ball with speed
        ball.update(&mut term)?;
        // draw all
        paddle_left.draw(&mut term)?;
        paddle_right.draw(&mut term)?;
        ball.draw(&mut term)?;
        term.update()?;
    }
    Ok(())
}

pub fn run() -> Result<(), AppError> {
    let term = TerminalImpl::new()?;
    let _ = game_loop(term)?;
    Ok(())
}
