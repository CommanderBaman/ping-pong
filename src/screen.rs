use std::io::{self, Write};

use crate::model::{Coordinate, Point};

#[derive(Debug)]
pub struct OutOfBoundError;

pub trait Screen {
    fn clear(&self) {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }
    fn draw(&self, coordinate: &Coordinate, character: char) -> Result<(), OutOfBoundError>;
}

pub struct TmuxScreen {
    height: Point,
    width: Point,
}

impl TmuxScreen {
    pub fn new() -> Self {
        let height = 80;
        let width = 120;
        assert!(height > 0);
        assert!(width > 0);
        Self { height, width }
    }
    fn is_bound(&self, coordinate: &Coordinate) -> Result<(), OutOfBoundError> {
        if coordinate.x > self.width {
            return Err(OutOfBoundError);
        }
        if coordinate.y > self.height {
            return Err(OutOfBoundError);
        }
        Ok(())
    }
    fn is_edge(&self, coordinate: &Coordinate) -> Option<Edge> {
        match (coordinate.x, coordinate.y) {
            (0, _) => Some(Edge::LEFT),
            (x, _) if x == self.width => Some(Edge::RIGHT),
            (_, y) if y == self.height => Some(Edge::BOTTOM),
            (_, 0) => Some(Edge::TOP),
            _ => None,
        }
    }
}
impl Screen for TmuxScreen {
    fn draw(&self, coordinate: &Coordinate, character: char) -> Result<(), OutOfBoundError> {
        self.is_bound(coordinate)?;
        print!(
            "\x1B[{};{}H{}",
            coordinate.y + 1,
            coordinate.x + 1,
            character
        );
        io::stdout().flush().unwrap();
        Ok(())
    }
}

enum Edge {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
}
