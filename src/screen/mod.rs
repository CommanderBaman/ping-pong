use std::time::Duration;

use crossterm::event::{Event, poll, read};

use crate::{AppError, config::Config, terminal::Terminal};

mod game;
mod settings;

fn wait_for_key_press() -> Result<(), AppError> {
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

pub fn screen<T: Terminal>(term: &mut T, config: &mut Config) -> Result<(), AppError> {
    loop {
        term.clear()?;
        let mut intro = String::new();
        intro += "Welcome to Ping Pong!\n";
        intro += "Press n for [N]ew game\n";
        intro += "Press s for [S]ettings\n";
        intro += "Press q for [Q]uit";
        term.print_mid(&intro, None)?;
        if poll(Duration::from_secs(5))? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('q') => break,
                    Some('n') => game::screen(term, config)?,
                    Some('s') => settings::screen(term, config)?,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}
