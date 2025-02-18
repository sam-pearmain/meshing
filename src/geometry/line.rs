// #![allow(dead_code)]

// use super::prelude::Float;
// use super::Dimensioned;
// use super::Point;
// use super::vertices::Vertex;

// pub struct Line<'a, F: Float, P: Point<F>> {
//     pub start: &'a Vertex<F, P>,
//     pub end: &'a Vertex<F, P>,
// }

// impl<'a, F: Float, P: Point<F>> Dimensioned for Line<'a, F, P> {
//     fn is_2d(&self) -> bool {
//         self.start.is_2d() && self.end.is_2d()
//     }
// }

// impl<'a, F: Float, P: Point<F>> Line<'a, F, P> {
//     pub fn between_vertices(v1: &'a Vertex<F, P>, v2: &'a Vertex<F, P>) -> Self {
//         Line {
//             start: v1,
//             end: v2,
//         }
//     }

//     pub fn length(&self) -> F {
//         if self.is_2d() {
//             let dx = self.end.x - self.start.x;
//             let dy = self.end.y - self.start.y;
//             (dx.powi(2) + dy.powi(2)).sqrt()
//         } else {
//             let dx = self.end.x - self.start.x;
//             let dy = self.end.y - self.start.y;
//             let dz = self.end.z - self.start.z;
//             (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
//         }
//     }
// }