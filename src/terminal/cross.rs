use std::io::{self, Write};

use crossterm::{ExecutableCommand, QueueableCommand, cursor, queue, style, terminal};

use crate::{
    model::Coordinate,
    terminal::{Edge, Terminal, TerminalError},
};

pub struct CrossTerminal {
    writer: io::Stdout,
    size: Coordinate,
}

impl CrossTerminal {
    pub fn new() -> Result<Self, TerminalError> {
        let mut writer = io::stdout();
        queue!(writer, terminal::EnterAlternateScreen, cursor::Hide)?;
        writer.flush()?;
        terminal::enable_raw_mode()?;
        let size = terminal::size()?;
        Ok(Self {
            writer,
            size: Coordinate::new(size.0, size.1),
        })
    }
}

impl Terminal for CrossTerminal {
    fn size(&self) -> &Coordinate {
        &self.size
    }
    fn clear(&mut self) -> Result<(), TerminalError> {
        self.writer
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
    fn draw(&mut self, coordinate: &Coordinate, character: char) -> Result<(), TerminalError> {
        queue!(
            self.writer,
            cursor::MoveTo(coordinate.x, coordinate.y),
            style::Print(character)
        )?;
        Ok(())
    }
    fn update(&mut self) -> Result<(), TerminalError> {
        self.writer.flush()?;
        Ok(())
    }
    fn edge(&self, coordinate: &Coordinate) -> Option<Edge> {
        match (coordinate.x, coordinate.y) {
            (0, _) => Some(Edge::LEFT),
            (x, _) if x >= self.size.x => Some(Edge::RIGHT),
            (_, y) if y >= self.size.y => Some(Edge::BOTTOM),
            (_, 0) => Some(Edge::TOP),
            _ => None,
        }
    }
    fn print(&mut self, value: &str) -> Result<(), TerminalError> {
        for c in value.chars() {
            self.writer.queue(style::Print(c))?;
        }
        self.update()?;
        Ok(())
    }
}

impl Drop for CrossTerminal {
    fn drop(&mut self) {
        let _ = self.writer.queue(terminal::LeaveAlternateScreen);
        let _ = self.writer.flush();
        let _ = terminal::disable_raw_mode();
    }
}
