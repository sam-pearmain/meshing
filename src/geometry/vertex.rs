use num_traits::Float;

pub struct Vertex<T: Float> {
    pub id: i32,
    pub x: T,
    pub y: T,
}