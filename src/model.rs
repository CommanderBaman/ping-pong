use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul},
};

pub type Point = u16;
pub type Real = f32;
pub type Score = u8;

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
impl Sqrt for Real {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sqrt> Vector<T> {
    pub fn magnitude(&self) -> T {
        let magnitude = (self.x * self.x) + (self.y * self.y);
        magnitude.sqrt()
    }
    pub fn norm(mut self) -> Self {
        let length = self.magnitude();
        self.x = self.x / length;
        self.y = self.y / length;
        self
    }
    pub fn power(mut self, p: T) -> Self {
        self.x = self.x * p;
        self.y = self.y * p;
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

pub type Coordinate = Vector<Point>;
pub type Position = Vector<Real>;
pub type Velocity = Vector<Real>;

impl From<Position> for Coordinate {
    fn from(value: Position) -> Self {
        Coordinate::new(value.x as Point, value.y as Point)
    }
}

impl From<Coordinate> for Position {
    fn from(value: Coordinate) -> Self {
        Position::new(value.x as Real, value.y as Real)
    }
}
