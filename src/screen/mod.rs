use std::time::Duration;

use crossterm::event::{Event, poll, read};

use crate::{AppError, terminal::Terminal};

mod game;

pub fn screen<T: Terminal>(term: &mut T) -> Result<(), AppError> {
    loop {
        term.clear()?;
        term.println("Welcome to Ping Pong!")?;
        term.println("Press n for [N]ew game")?;
        term.println("Press q for [Q]uit")?;
        if poll(Duration::from_secs(5))? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('q') => break,
                    Some('n') => game::screen(term)?,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}
