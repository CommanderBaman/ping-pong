mod config;
mod drawable;
mod model;
mod screen;
mod terminal;

use thiserror::Error;

use crate::{
    config::Config,
    screen::screen,
    terminal::{TerminalError, TerminalImpl},
};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Terminal(#[from] TerminalError),
}

pub fn run() -> Result<(), AppError> {
    let mut term = TerminalImpl::new()?;
    let mut config = Config::new();
    screen(&mut term, &mut config)?;
    Ok(())
}
