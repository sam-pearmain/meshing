use super::prelude::*;
use super::Vertex;

pub struct Line<'a, T: Float> {
    pub start: &'a Vertex<T>,
    pub end: &'a Vertex<T>,
}

impl<'a, T: Float> Line<'a, T> {
    pub fn new(v1: &'a Vertex<T>, v2: &'a Vertex<T>) -> Line<'a, T> {
        Line {
            start: v1,
            end: v2,
        }
    }

    pub fn length(&self) -> T {
        let x_diff: T = self.end.coords.x - self.start.coords.x;
        let y_diff: T = self.end.coords.y - self.start.coords.y;
        let z_diff: T = self.end.coords.z - self.start.coords.z;
        (x_diff.powi(2) + y_diff.powi(2) + z_diff.powi(2)).sqrt()
    }
}