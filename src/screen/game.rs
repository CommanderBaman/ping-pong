use std::time::Duration;

use crate::{
    AppError,
    config::Config,
    drawable::{Ball, Drawable, Paddle},
    model::{Coordinate, Point, Score},
    screen::wait_for_key_press,
    terminal::{Edge, Terminal, TerminalError},
};
use crossterm::event::{Event, poll, read};

struct ScoreBoard {
    pub left: Score,
    pub right: Score,
    max_score: Score,
    points_per_goal: Score,
    mid: Coordinate,
    pub last_scored: Option<bool>,
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
            last_scored: None,
        }
    }
    pub fn score_right(&mut self) {
        self.last_scored = Some(false);
        self.right += self.points_per_goal;
    }
    pub fn score_left(&mut self) {
        self.last_scored = Some(true);
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
            let coordinate = Coordinate::new(self.mid.x - (i as Point) - 1, self.mid.y);
            term.draw(&coordinate, c)?;
        }
        term.draw(&self.mid, ':')?;
        for (i, c) in self.right.to_string().chars().enumerate() {
            let coordinate = Coordinate::new(self.mid.x + (i as Point) + 1, self.mid.y);
            term.draw(&coordinate, c)?;
        }
        Ok(())
    }
}

pub fn screen<T: Terminal>(term: &mut T, config: &Config) -> Result<(), AppError> {
    let mut score_board = ScoreBoard::new(term, config);
    let mut stopped = false;
    while !stopped && score_board.winner().is_none() {
        term.clear()?;
        if let Some(left_scored) = score_board.last_scored {
            if left_scored {
                term.print_mid("Left scored", Some(-1))?;
            } else {
                term.print_mid("Right scored", Some(-1))?;
            }
            term.print_mid("Press any key to continue", Some(0))?;
            wait_for_key_press()?;
        }
        stopped = game_loop(term, config, &mut score_board)?;
    }
    term.clear()?;
    let mut result_string = String::new();
    if stopped {
        result_string += "game stopped\n";
    }
    let board_string = format!("{}", score_board);
    result_string += &board_string;
    result_string += "\npress any key to go back to home screen...";
    term.print_mid(&result_string, None)?;
    wait_for_key_press()?;
    Ok(())
}

fn game_loop<T: Terminal>(
    term: &mut T,
    config: &Config,
    score_board: &mut ScoreBoard,
) -> Result<bool, AppError> {
    term.clear()?;
    let term_size = term.size();
    let mut paddle_left = Paddle::new(config.paddle_length, 0, config.paddle_speed);
    let mut paddle_right = Paddle::new(config.paddle_length, term_size.x, config.paddle_speed);
    let mut ball = Ball::new(config);
    let frame_time = Duration::from_millis(1000 / config.frames_per_second);
    // place ball in middle
    ball.place(Coordinate::new(term_size.x / 2, term_size.y / 2));
    while score_board.winner().is_none() {
        if poll(frame_time)? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('d') => paddle_left.move_down(term)?,
                    Some('f') => paddle_left.move_up(term)?,
                    Some('j') => paddle_right.move_down(term)?,
                    Some('k') => paddle_right.move_up(term)?,
                    Some('q') => return Ok(true),
                    Some('b') => {
                        term.print_mid("BREAK!", Some(0))?;
                        wait_for_key_press()?;
                        // erasing the break
                        term.print_mid("      ", Some(0))?;
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
                score_board.score_right();
                break;
            }
            Some(Edge::RIGHT) => {
                score_board.score_left();
                break;
            }
        }
        // draw all
        score_board.draw(term)?;
        ball.draw(term)?;
        paddle_left.draw(term)?;
        paddle_right.draw(term)?;
        term.update()?;
    }
    Ok(false)
}
