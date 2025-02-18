use super::prelude::Float;

/// marks types that have a dimensional property
pub trait Dimensioned {
    /// returns true/false if 2D or not 2D
    fn is_2d(&self) -> bool;

    /// get the number of dimensions
    fn dimensions(&self) -> usize {
        if self.is_2d() { 2 } else { 3 }
    }
}

/// core trait for geometric points in 2D or 3D space
pub trait Point<F: Float>: Dimensioned + Clone + PartialEq {
    /// returns coordinates as a tuple (either (F, F) or (F, F, F))
    type Coordinates: Clone;

    /// creates a new point at the origin
    fn origin() -> Self;

    /// creates a point from given coordinates
    fn from_coordinates(coords: Self::Coordinates) -> Self;

    /// returns the point's coordiantes
    fn coordinates(&self) -> Self::Coordinates;
}