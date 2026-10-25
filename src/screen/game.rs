use std::time::Duration;

use crate::{
    AppError,
    config::Config,
    drawable::{Ball, Drawable, Paddle},
    model::Coordinate,
    terminal::{Edge, Terminal, TerminalError},
};
use crossterm::event::{Event, poll, read};

struct ScoreBoard {
    pub left: u8,
    pub right: u8,
    max_score: u8,
    points_per_goal: u8,
    mid: Coordinate,
}

impl ScoreBoard {
    pub fn new<T: Terminal>(term: &T, config: &Config) -> Self {
        let size = term.size();
        let mid = Coordinate::new(size.x / 2, 0);
        Self {
            left: 0,
            right: 0,
            mid,
            max_score: config.max_score,
            points_per_goal: config.points_per_goal,
        }
    }
    pub fn score_left(&mut self) {
        self.right += self.points_per_goal;
    }
    pub fn score_right(&mut self) {
        self.left += self.points_per_goal;
    }
    pub fn winner(&self) -> Option<bool> {
        // true means left won
        if self.left >= self.max_score {
            return Some(true);
        }
        if self.right >= self.max_score {
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

pub fn screen<T: Terminal>(term: &mut T, config: &Config) -> Result<(), AppError> {
    let mut score_board = ScoreBoard::new(term, config);
    let _ = game_loop(term, config, &mut score_board)?;
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

fn game_loop<T: Terminal>(
    term: &mut T,
    config: &Config,
    score_board: &mut ScoreBoard,
) -> Result<(), AppError> {
    term.clear()?;
    let term_size = term.size();
    let mut paddle_left = Paddle::new(config.paddle_length, 0, config.paddle_speed);
    let mut paddle_right = Paddle::new(config.paddle_length, term_size.x, config.paddle_speed);
    let mut ball = Ball::new(config);
    let frame_time = Duration::from_millis(1000 / config.frames_per_second);
    // place ball in middle
    ball.place(Coordinate::new(term_size.x / 2, term_size.y / 2));
    loop {
        if poll(frame_time)? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('d') => paddle_left.move_down(term)?,
                    Some('f') => paddle_left.move_up(term)?,
                    Some('j') => paddle_right.move_down(term)?,
                    Some('k') => paddle_right.move_up(term)?,
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
            ball.bounce_horizontal();
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
