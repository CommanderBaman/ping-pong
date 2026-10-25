use std::{fmt::Display, str::FromStr, time::Duration};

use crossterm::event::{Event, poll, read};

use crate::{
    AppError,
    config::Config,
    terminal::{Terminal, TerminalError},
};

fn get_settings_string(config: &Config) -> String {
    let mut settings = String::new();
    settings += &format!("[1] frames per second = {}\n", config.frames_per_second);
    settings += &format!("[2] paddle length = {}\n", config.paddle_length);
    settings += &format!("[3] paddle speed = {}\n", config.paddle_speed);
    settings += &format!("[4] max score = {}\n", config.max_score);
    settings += &format!("[5] points per goal = {}\n", config.points_per_goal);
    settings += &format!(
        "[6] bounce ball randomly instead of reflecting = {}\n",
        config.random_bounce
    );
    settings += &format!("[7] initial ball speed = {}\n", config.initial_ball_speed);
    settings += &format!(
        "[8] ball speed multiplier = {}\n",
        config.ball_speed_multiplier
    );
    settings
}

fn select_value<T: Terminal, A: FromStr + Display>(
    term: &mut T,
    name: &str,
    value: A,
) -> Result<Option<A>, TerminalError> {
    let mut new_value = None;
    let mut line = String::new();
    loop {
        term.clear()?;
        term.println("Enter q to [q]uit change")?;
        if !line.is_empty() {
            let s = format!("invalid value entered: {}", line);
            term.println(&s)?;
        }
        let query = format!("Enter value for {} ({})", name, value);
        term.println(&query)?;
        line = term.read_line()?;
        // break if q
        if matches!(line.chars().next(), Some('q')) && line.chars().next().is_none() {
            break;
        }
        // break if got number
        if let Ok(v) = line.parse::<A>() {
            new_value = Some(v);
            break;
        }
    }
    Ok(new_value)
}

pub fn screen<T: Terminal>(term: &mut T, config: &mut Config) -> Result<(), AppError> {
    loop {
        term.clear()?;
        let mut settings = String::new();
        settings += "Select the setting you want to change\n";
        settings += &get_settings_string(config);
        settings += "press h for [H]ome screen";
        term.println(&settings)?;
        if poll(Duration::from_secs(5))? {
            match read()? {
                Event::Key(event) => match event.code.as_char() {
                    Some('1') => {
                        if let Some(value) = select_value(
                            term,
                            "frames per second",
                            config.frames_per_second as u16,
                        )? {
                            config.frames_per_second = value as u64;
                        }
                    }
                    Some('2') => {
                        if let Some(value) =
                            select_value(term, "paddle length", config.paddle_length)?
                        {
                            config.paddle_length = value;
                        }
                    }
                    Some('3') => {
                        if let Some(value) =
                            select_value(term, "paddle speed", config.paddle_speed)?
                        {
                            config.paddle_speed = value;
                        }
                    }
                    Some('4') => {
                        if let Some(value) = select_value(term, "max score", config.max_score)? {
                            config.max_score = value;
                        }
                    }
                    Some('5') => {
                        if let Some(value) =
                            select_value(term, "points per goal", config.points_per_goal)?
                        {
                            config.points_per_goal = value;
                        }
                    }
                    Some('6') => {
                        if let Some(value) =
                            select_value(term, "random bounce", config.random_bounce)?
                        {
                            config.random_bounce = value;
                        }
                    }
                    Some('7') => {
                        if let Some(value) =
                            select_value(term, "initial ball speed", config.initial_ball_speed)?
                        {
                            config.initial_ball_speed = value;
                        }
                    }
                    Some('8') => {
                        if let Some(value) = select_value(
                            term,
                            "ball speed multiplier",
                            config.ball_speed_multiplier,
                        )? {
                            config.ball_speed_multiplier = value;
                        }
                    }
                    Some('h') => break,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}
