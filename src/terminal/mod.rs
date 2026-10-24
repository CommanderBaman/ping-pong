use thiserror::Error;

use crate::{constants, model::Coordinate};
mod cross;
mod diy;

#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("out of bounds at {0}")]
    OutOfBound(Coordinate),
}

pub trait Terminal {
    fn clear(&mut self) -> Result<(), TerminalError>;
    fn update(&mut self) -> Result<(), TerminalError>;
    fn draw(&mut self, coordinate: &Coordinate, character: char) -> Result<(), TerminalError>;
    fn size(&self) -> &Coordinate;
    fn edge(&self, coordinate: &Coordinate) -> Option<Edge>;
    fn erase(&mut self, coordinate: &Coordinate) -> Result<(), TerminalError> {
        self.draw(coordinate, constants::EMPTY_CHARACTER)
    }
    fn println(&mut self, value: &str) -> Result<(), TerminalError>;
}

pub enum Edge {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
}

pub type TerminalImpl = cross::CrossTerminal;
