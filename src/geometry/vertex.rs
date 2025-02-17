#![allow(dead_code)]

use crate::utils::error::*;
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

impl<T: Float + fmt::Display> fmt::Display for Vertex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, "vertex {} at ({}, {}, {})",
            self.id, self.coords.x, self.coords.y, self.coords.z
        )
    }
}

pub enum Dimensions {
    TwoDimensions, 
    ThreeDimensions,
}

pub enum WriteOrder {
    IJK, // loop in x, then y, then k
    JIK, // loop in y, then x, then k
}

pub struct VertexCollection<T: Float> {
    vertices: Vec<Vertex<T>>,
    dimensions: Dimensions,
    write_order: WriteOrder, 
}

impl<T: Float + fmt::Display> VertexCollection<T> {
    pub fn new(dimensions: Dimensions, write_order: WriteOrder) -> Self {
        VertexCollection {
            vertices: Vec::new(),
            dimensions,
            write_order,
        }
    }

    pub fn find_vertex(&self, id: i32) -> Result<&Vertex<T>, GeometryError> {
        for vertex in &self.vertices {
            if vertex.id == id {
                return Ok(vertex);
            }
        }
        Err(GeometryError::VertexNotFound { vertex_id: id })
    }
}