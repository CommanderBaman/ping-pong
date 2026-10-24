use thiserror::Error;

use crate::model::Coordinate;
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
}

pub enum Edge {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
}

pub type TerminalImpl = cross::CrossTerminal;
