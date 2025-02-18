#![allow(dead_code)]

use super::{prelude::*, Dimensioned, Point};

#[derive(Debug, Clone, Copy)]
pub struct Point2D<T: Float> {
    x: T,
    y: T
}

impl<T: Float> Dimensioned for Point2D<T> {
    fn is_2d(&self) -> bool {
        true
    }
}

impl<T: Float> Point<T> for Point2D<T> {
    type Tuple = (T, T);
    
    fn origin() -> Self {
        Point2D {
            x: T::zero(),
            y: T::zero(),
        }
    }

    fn distance_to(&self, other: &Self) -> T {
        (
            (self.x - other.x).powi(2) + 
            (self.y - other.y).powi(2)
        ).sqrt()
    }

    fn as_tuple(&self) -> Self::Tuple {
        (self.x, self.y)
    }
}

impl<T: Float + fmt::Display> fmt::Display for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D ({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Dimensioned for Point3D<T> {
    fn is_2d(&self) -> bool {
        false
    }
}

impl<T: Float> Point<T> for Point3D<T> {
    type Tuple = (T, T, T);
    
    fn origin() -> Self {
        Point3D {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn distance_to(&self, other: &Self) -> T {
        (
            (self.x - other.x).powi(2) + 
            (self.y - other.y).powi(2) + 
            (self.z - other.z).powi(2)
        ).sqrt()
    }

    fn as_tuple(&self) -> Self::Tuple {
        (self.x, self.y, self.z)
    }
}

impl<T: Float + fmt::Display> fmt::Display for Point3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point3D({}, {}, {})", self.x, self.y, self.z)
    }
}