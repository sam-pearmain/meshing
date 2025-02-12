use num_traits::Float;

pub struct Vertex<T: Float> {
    id: i32,
    x: T,
    y: T,
}