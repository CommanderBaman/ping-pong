use std::time::Duration;

use crate::{
    AppError, constants,
    drawable::{Ball, Drawable, Paddle},
    model::Coordinate,
    terminal::{Edge, Terminal, TerminalError},
};
use crossterm::event::{Event, poll, read};

struct ScoreBoard {
    pub left: u8,
    pub right: u8,
    mid: Coordinate,
}

impl ScoreBoard {
    pub fn new<T: Terminal>(term: &T) -> Self {
        let size = term.size();
        let mid = Coordinate::new(size.x / 2, 0);
        Self {
            left: 0,
            right: 0,
            mid,
        }
    }
    pub fn score_left(&mut self) {
        self.right += constants::POINT_PER_GOAL;
    }
    pub fn score_right(&mut self) {
        self.left += constants::POINT_PER_GOAL;
    }
    pub fn winner(&self) -> Option<bool> {
        // true means left won
        if self.left >= constants::MAX_SCORE {
            return Some(true);
        }
        if self.right >= constants::MAX_SCORE {
            return Some(false);
        }
        None
    }
}

impl std::fmt::Display for ScoreBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = format!("left {} : right {}", self.left, self.right);
        if let Some(left_won) = self.winner() {
            if left_won {
                s += "\nLeft Won";
            } else {
                s += "\nRight Won";
            }
        }
        write!(f, "{}", s)
    }
}

impl<T: Terminal> Drawable<T> for ScoreBoard {
    fn clear(&self, _term: &mut T) -> Result<(), TerminalError> {
        unreachable!()
    }
    fn draw(&self, term: &mut T) -> Result<(), TerminalError> {
        for (i, c) in self.left.to_string().chars().rev().enumerate() {
            let coordinate = Coordinate::new(self.mid.x - (i as u16) - 1, self.mid.y);
            term.draw(&coordinate, c)?;
        }
        term.draw(&self.mid, ':')?;
        for (i, c) in self.right.to_string().chars().enumerate() {
            let coordinate = Coordinate::new(self.mid.x + (i as u16) + 1, self.mid.y);
            term.draw(&coordinate, c)?;
        }
        Ok(())
    }
}

pub fn screen<T: Terminal>(term: &mut T) -> Result<(), AppError> {
    let mut score_board = ScoreBoard::new(term);
    let _ = game_loop(term, &mut score_board)?;
    term.clear()?;
    let board_string = format!("{}", score_board);
    term.println(&board_string)?;
    term.println("press any key to go back to home screen...")?;
    loop {
        if poll(Duration::from_secs(5))? {
            match read()? {
                Event::Key(_) => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn game_loop<T: Terminal>(term: &mut T, score_board: &mut ScoreBoard) -> Result<(), AppError> {
    term.clear()?;
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

        // if ball colliding with paddles, reverse direction in x
        let ball_coordinate = ball.coordinate();
        if paddle_left.occupied(&ball_coordinate) || paddle_right.occupied(&ball_coordinate) {
            ball.reverse_horizontal();
            ball.update(term)?;
        }

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
        ball.draw(term)?;
        paddle_left.draw(term)?;
        paddle_right.draw(term)?;
        if score_board.winner().is_some() {
            break;
        }
        term.update()?;
    }
    Ok(())
}
