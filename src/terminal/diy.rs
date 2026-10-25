#![allow(dead_code, unused_variables)]

use std::io::{self, Write};

use crate::{
    model::{Coordinate, Point},
    terminal::{Edge, Terminal, TerminalError},
};

pub struct TmuxTerminal {
    height: Point,
    width: Point,
}

impl TmuxTerminal {
    pub fn new() -> Self {
        let height = 80;
        let width = 120;
        assert!(height > 0);
        assert!(width > 0);
        Self { height, width }
    }
    fn is_bound(&self, coordinate: &Coordinate) -> Result<(), TerminalError> {
        if coordinate.x > self.width {
            return Err(TerminalError::OutOfBound(coordinate.clone()));
        }
        if coordinate.y > self.height {
            return Err(TerminalError::OutOfBound(coordinate.clone()));
        }
        Ok(())
    }
}
impl Terminal for TmuxTerminal {
    fn erase(&mut self, coordinate: &Coordinate) -> Result<(), TerminalError> {
        unimplemented!()
    }
    fn println(&mut self, value: &str) -> Result<(), TerminalError> {
        unimplemented!()
    }
    fn update(&mut self) -> Result<(), TerminalError> {
        Ok(())
    }
    fn size(&self) -> &Coordinate {
        unimplemented!()
    }
    fn edge(&self, coordinate: &Coordinate) -> Option<Edge> {
        match (coordinate.x, coordinate.y) {
            (0, _) => Some(Edge::LEFT),
            (x, _) if x == self.width => Some(Edge::RIGHT),
            (_, y) if y == self.height => Some(Edge::BOTTOM),
            (_, 0) => Some(Edge::TOP),
            _ => None,
        }
    }
    fn clear(&mut self) -> Result<(), TerminalError> {
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;
        Ok(())
    }
    fn draw(&mut self, coordinate: &Coordinate, character: char) -> Result<(), TerminalError> {
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
    fn print_mid(&mut self, value: &str, shift: Option<i16>) -> Result<(), TerminalError> {
        unimplemented!()
    }
    fn read_line(&mut self) -> Result<String, TerminalError> {
        unimplemented!()
    }
}
