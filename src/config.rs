pub struct Config {
    pub paddle_length: u16,
    pub paddle_speed: u16,
    pub frames_per_second: u64,
    pub max_score: u8,
    pub points_per_goal: u8,
}

impl Config {
    pub fn new() -> Self {
        Self {
            paddle_length: 11,
            paddle_speed: 3,
            frames_per_second: 30,
            max_score: 2,
            points_per_goal: 1,
        }
    }
}
