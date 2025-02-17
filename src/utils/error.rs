use std::fmt::{Display, Result, Formatter};
use num_traits::Float;

use crate::geometry::Vertex;

#[derive(Debug)]
pub enum GeometryError {
    IncompleteNode{ node_id: i32 } 
}

#[derive(Debug)]
pub enum MeshError<T: Float> {
    EmptyMesh,
    DuplicateVertexIds{ v1: Vertex<T>, v2: Vertex<T> },
}

impl Display for GeometryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GeometryError::IncompleteNode { node_id } => {
                write!(f, "node {} is incomplete", node_id)
            }
        }
    }
}

impl<T: Float + Display> Display for MeshError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            MeshError::EmptyMesh => {
                write!(f, "mesh is empty")
            }
            MeshError::DuplicateVertexIds { v1, v2 } => {
                write!(f, "duplicate vertices found: {}, {}", v1, v2)
            }
        }
    }
}