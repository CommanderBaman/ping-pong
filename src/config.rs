use crate::model::{Point, Real, Score};

pub struct Config {
    pub paddle_length: Point,
    pub paddle_speed: Point,
    pub frames_per_second: u64,
    pub max_score: Score,
    pub points_per_goal: Score,
    pub random_bounce: bool,
    pub initial_ball_speed: Real,
    pub ball_speed_multiplier: Real,
}

impl Config {
    pub fn new() -> Self {
        Self {
            paddle_length: 11,
            paddle_speed: 3,
            frames_per_second: 30,
            max_score: 2,
            points_per_goal: 1,
            random_bounce: false,
            initial_ball_speed: 2.0,
            ball_speed_multiplier: 1.0,
        }
    }
}
