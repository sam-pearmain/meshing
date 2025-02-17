use std::{fmt, i32};
use num_traits::Float;

use crate::geometry::{Node2D, Vertex};

#[derive(Debug)]
pub enum GeometryError {
    IncompleteNode{ node_id: i32 } 
}

#[derive(Debug)]
pub enum MeshError<T: Float> {
    EmptyMesh,
    DuplicateVertexIds{ v1: Vertex<T>, v2: Vertex<T> },
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeometryError::IncompleteNode { node_id: i32 } => {
                write!(f, "node {} is incomplete", node_id)
            }
        }
    }
}

impl<T: Float + fmt::Display> fmt::Display for MeshError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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