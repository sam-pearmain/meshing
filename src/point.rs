use std::fmt::Display;

use num::{Num, Signed};

pub trait Coordinate: Num + Clone + Copy + Default + std::fmt::Debug {}

impl Coordinate for f32   {}
impl Coordinate for f64   {}
impl Coordinate for i8    {}
impl Coordinate for i16   {}
impl Coordinate for i32   {}
impl Coordinate for i64   {}
impl Coordinate for i128  {}
impl Coordinate for u8    {}
impl Coordinate for u16   {}
impl Coordinate for u32   {}
impl Coordinate for u64   {}
impl Coordinate for u128  {}
impl Coordinate for usize {}

#[derive(PartialEq, PartialOrd)]
pub struct Point<T: Coordinate, const DIMS: usize> {
    coords: [T; DIMS],
}

impl<T: Coordinate, const DIMS: usize> std::fmt::Debug for Point<T, DIMS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "point::{}, {:?}", std::any::type_name::<T>(), &self.coords)
    }
}

impl<T: Coordinate + Display, const DIMS: usize> std::fmt::Display for Point<T, DIMS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coords_string = self.coords.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "({})", coords_string)
    }
}

type Point1D<T> = Point<T, 1>;
type Point2D<T> = Point<T, 2>;
type Point3D<T> = Point<T, 3>;

impl<T: Coordinate> Point1D<T> {
    pub fn x(&self) -> T { self.coords[0] }
}

impl<T: Coordinate> Point2D<T> {
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
}

impl<T: Coordinate> Point3D<T> {
    pub fn x(&self) -> T { self.coords[0] }
    pub fn y(&self) -> T { self.coords[1] }
    pub fn z(&self) -> T { self.coords[2] }
}

impl<T: Coordinate, const DIMS: usize> Point<T, DIMS> {
    pub fn new() -> Self {
        Point { coords: [T::default(); DIMS] }
    }

    pub fn origin() -> Self {
        Point { coords: [T::zero(); DIMS] }
    }

    pub fn from(coords: impl Into<[T; DIMS]>) -> Self {
        Point { coords: coords.into() }
    }
}

impl<T: Coordinate, const DIMS: usize> std::ops::Add for Point<T, DIMS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coords = [T::default(); DIMS];
        for (i, (&a, &b)) in self.coords.iter().zip(rhs.coords.iter()).enumerate() {
            coords[i] = a + b;
        }
        Point { coords }
    }
}

impl<T: Coordinate + Signed, const DIMS: usize> std::ops::Sub for Point<T, DIMS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut coords = [T::default(); DIMS];
        for (i, (&a, &b)) in self.coords.iter().zip(rhs.coords.iter()).enumerate() {
            coords[i] = a - b;
        }
        Point { coords }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let point = Point::<i32, 3>::from([1, 2, 3]);
        assert_eq!(point.coords, [1, 2, 3]);
    }

    #[test]
    fn test_point_origin() {
        let origin = Point::<i32, 3>::origin();
        assert_eq!(origin.coords, [0, 0, 0]);
    }

    #[test]
    fn test_point_addition() {
        let p1 = Point::<i32, 3>::from([1, 2, 3]);
        let p2 = Point::<i32, 3>::from([4, 5, 6]);
        let result = p1 + p2;
        assert_eq!(result.coords, [5, 7, 9]);
    }

    #[test]
    fn test_point_subtraction() {
        let p1 = Point3D::<i32>::from([4, 5, 6]);
        let p2 = Point3D::<i32>::from([1, 2, 3]);
        let result = p1 - p2;
        assert_eq!(result.coords, [3, 3, 3]);
    }

    #[test]
    fn test_point_display() {
        let point = Point::<i32, 3>::from([1, 2, 3]);
        assert_eq!(format!("{}", point), "(1, 2, 3)");
    }

    #[test]
    fn test_point_from_tuple() {
        let point = Point::<i32, 2>::from((4, 5));
        assert_eq!(point.coords, [4, 5]);
    }
}