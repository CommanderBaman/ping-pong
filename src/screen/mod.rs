use std::time::Duration;

use crossterm::event::poll;

use crate::{
    AppError, constants,
    drawable::Drawable,
    model::Coordinate,
    terminal::{Terminal, TerminalError},
};

mod game;

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
        todo!()
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

pub fn screen_loop<T: Terminal>(term: &mut T) -> Result<(), AppError> {
    let mut score_board = ScoreBoard::new(term);
    let _ = game::screen_loop(term, &mut score_board)?;
    term.clear()?;
    let board_string = format!("{}", score_board);
    term.println(&board_string)?;
    term.println("auto exiting after 5 seconds")?;
    term.println("press any key to exit...")?;
    if poll(Duration::from_secs(5))? {}
    Ok(())
}
