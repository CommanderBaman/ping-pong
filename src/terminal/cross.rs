use std::io::{self, Write};

use crossterm::{ExecutableCommand, QueueableCommand, cursor, queue, style, terminal};

use crate::{
    model::Coordinate,
    terminal::{Terminal, TerminalError},
};

pub struct CrossTerminal {
    writer: io::Stdout,
}

impl CrossTerminal {
    pub fn new() -> Result<Self, TerminalError> {
        let mut writer = io::stdout();
        queue!(writer, terminal::EnterAlternateScreen, cursor::Hide)?;
        writer.flush()?;
        terminal::enable_raw_mode()?;
        Ok(Self { writer })
    }
}

impl Terminal for CrossTerminal {
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
        self.writer.flush()?;
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
