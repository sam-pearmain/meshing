#![allow(dead_code)]

use std::fmt::{Display, Result, Formatter};
use num_traits::Float;
use crate::geometry::{Direction, Vertex};

#[derive(Debug)]
pub enum GeometryError<T: Float + Display> {
    IncompleteNode{ node_id: u32 }, // should return information about the bad node not just an id
    VertexNotFound{ vertex_id: u32},
    InvalidDirection{ direction: Direction },
    BoundaryVertex{ vertex_id: u32, direction: Direction},
    InvalidVertexID,
    VertexLimitExceeded{ limit: u32, attempted: u32 },
    InvalidVertexCoordinate{ coordinate: char, expected: T, received: T }
}

#[derive(Debug)]
pub enum MeshError<T: Float> {
    EmptyMesh,
    DuplicateVertexIds{ v1: Vertex<T>, v2: Vertex<T> },
}

impl<T: Float + Display> Display for GeometryError<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GeometryError::IncompleteNode { node_id } => {
                write!(f, "node: {} is incomplete", node_id)
            }
            GeometryError::VertexNotFound { vertex_id } => {
                write!(f, "vertex: {} not found", vertex_id)
            }
            GeometryError::InvalidDirection { direction } => {
                write!(f, "{} not possible for 2D mesh", direction)
            }
            GeometryError::BoundaryVertex { vertex_id, direction } => {
                write!(f, "vertex: {} is a boundary vertex, nothing exists {} of it", vertex_id, direction)
            }
            GeometryError::InvalidVertexID => {
                write!(f, "vertex must have non zero id")
            }
            GeometryError::VertexLimitExceeded { limit, attempted } => {
                write!(f, "vertex limit exceeded {} of {}", attempted, limit)
            }
            GeometryError::InvalidVertexCoordinate { coordinate, expected, received } => {
                write!(f, "{}-coordinate must be {} for a 2D mesh, received {}", coordinate, expected, received)
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