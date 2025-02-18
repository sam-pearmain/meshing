use super::prelude::Float;

pub trait Dimensioned {
    fn is_2d(&self) -> bool;
}

pub trait Point<F: Float>: Dimensioned {
    type Tuple;
    
    fn origin() -> Self;
    fn distance_to(&self, other: &Self) -> F;
    fn as_tuple(&self) -> Self::Tuple;
    fn get_x(&self) -> F;
    fn get_y(&self) -> F;
    fn get_z(&self) -> Option<F>;
}

pub trait Vertex<F: Float>: Point<F> {
    fn new<P: Point<F>>(id: usize, p: P) -> Self;
    fn id(&self) -> usize;
    fn set_id(&self, id: usize) -> ();
}