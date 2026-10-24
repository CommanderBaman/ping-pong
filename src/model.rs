use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul},
};

pub type Point = u16;

#[derive(Debug, Clone, Copy)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sqrt> Vector<T> {
    pub fn norm(mut self) -> Self {
        let length = (self.x * self.x) + (self.y * self.y);
        let length = length.sqrt();
        self.x = self.x / length;
        self.y = self.y / length;
        self
    }
}

impl<T: AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Display> Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub type Coordinate = Vector<u16>;
pub type Position = Vector<f32>;
pub type Velocity = Vector<f32>;

impl From<Position> for Coordinate {
    fn from(value: Position) -> Self {
        Coordinate::new(value.x as u16, value.y as u16)
    }
}

impl From<Coordinate> for Position {
    fn from(value: Coordinate) -> Self {
        Position::new(value.x as f32, value.y as f32)
    }
}
