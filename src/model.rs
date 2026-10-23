pub type Point = u16;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: Point,
    pub y: Point,
}

impl Vector {
    pub fn new(x: Point, y: Point) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub type Coordinate = Vector;
pub type Velocity = Vector;
