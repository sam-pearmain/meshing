use super::prelude::Float;

pub trait Dimensioned {
    fn is_2d(&self) -> bool;
}

pub trait Point<T: Float>: Dimensioned {
    type Tuple;
    
    fn origin() -> Self;
    fn distance_to(&self, other: &Self) -> T;
    fn as_tuple(&self) -> Self::Tuple;
}

pub trait Vertex<T: Float>: Point<T> {
    fn new<P: Point<T>>(id: usize, p: P) -> Self;
    fn id(&self) -> usize;
    fn set_id(&self, id: usize) -> ();
}