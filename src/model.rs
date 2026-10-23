pub type Point = u16;
pub struct Vector {
    pub x: Point,
    pub y: Point,
}
impl Vector {
    pub fn new(x: Point, y: Point) -> Self {
        Self { x, y }
    }
}

pub type Coordinate = Vector;
pub type Velocity = Vector;
