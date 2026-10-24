use std::time::Duration;

use crate::{
    AppError, constants,
    drawable::{Ball, Drawable, Paddle},
    model::Coordinate,
    screen::ScoreBoard,
    terminal::{Edge, Terminal},
};
use crossterm::event::{Event, poll, read};

pub fn screen_loop<T: Terminal>(
    term: &mut T,
    score_board: &mut ScoreBoard,
) -> Result<(), AppError> {
    let term_size = term.size();
    let mut paddle_left = Paddle::new(constants::PADDLE_LENGTH, 0);
    let paddle_right = Paddle::new(constants::PADDLE_LENGTH, term_size.x);
    let mut ball = Ball::new();
    let frame_time = Duration::from_millis(1000 / constants::FRAMES_PER_SECOND);
    // place ball in middle
    ball.place(Coordinate::new(term_size.x / 2, term_size.y / 2));
    loop {
        if poll(frame_time)? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('k') => paddle_left.move_up(term)?,
                    Some('j') => paddle_left.move_down(term)?,
                    Some('q') => {
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        // update the ball with speed
        ball.update(term)?;
        // if ball at edge, score
        match term.edge(&ball.coordinate()) {
            None | Some(Edge::TOP) | Some(Edge::BOTTOM) => {}
            Some(Edge::LEFT) => {
                score_board.score_left();
            }
            Some(Edge::RIGHT) => score_board.score_right(),
        }
        // draw all
        score_board.draw(term)?;
        paddle_left.draw(term)?;
        paddle_right.draw(term)?;
        ball.draw(term)?;
        if score_board.winner().is_some() {
            break;
        }
        term.update()?;
    }
    Ok(())
}
