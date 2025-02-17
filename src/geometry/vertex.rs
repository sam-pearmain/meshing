use super::prelude::*;
use super::Point3D;

#[derive(Debug)]
pub struct Vertex<T: Float> {
    pub id: i32,
    pub coords: Point3D<T>,
}

impl<T: Float> Vertex<T> {
    pub fn new(id: i32, x: T, y: T, z: T) -> Vertex<T> {
        Vertex {
            id: id, 
            coords: Point3D::new(x, y, z),
        }
    }
}

impl<T: Float + Display> Display for Vertex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "vertex {} at ({}, {}, {})",
            self.id, self.coords.x, self.coords.y, self.coords.z
        )
    }
}
