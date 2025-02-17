#![allow(dead_code)]

use std::fmt::Display;

use crate::utils::error::*;
use super::prelude::*;
use super::Point3D;

#[derive(Debug)]
pub struct Vertex<T: Float> {
    pub id: u32,
    pub coords: Point3D<T>,
}

impl<T: Float> Vertex<T> {
    pub fn new(id: u32, x: T, y: T, z: T) -> Result<Vertex<T>, GeometryError> {
        if id == 0 {
            return Err(GeometryError::InvalidVertexID);    
        }
        Ok(Vertex {
            id: id, 
            coords: Point3D::new(x, y, z),
        })
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,  // +y 
    South,  // -y
    East,   // +x
    West,   // -x
    Up,     // +z
    Down,   // -z
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::South => write!(f, "south"),
            Direction::East  => write!(f, "east"),
            Direction::West  => write!(f, "west"),
            Direction::Up    => write!(f, "up"),
            Direction::Down  => write!(f, "down"),
        }
    }
}

pub enum Dimensions {
    TwoDimensions{ nx: u32, ny: u32 }, 
    ThreeDimensions{ nx: u32, ny: u32, nz: u32 },
}

impl Dimensions {
    pub fn get_nx(&self) -> u32 {
        match self {
            Dimensions::TwoDimensions { nx, .. } => *nx,
            Dimensions::ThreeDimensions { nx, .. } => *nx,
        }
    }

    pub fn get_ny(&self) -> u32 {
        match self {
            Dimensions::TwoDimensions { ny, .. } => *ny,
            Dimensions::ThreeDimensions { ny, .. } => *ny,
        }
    }

    pub fn get_nz(&self) -> u32 {
        match self {
            Dimensions::TwoDimensions { .. } => 1,
            Dimensions::ThreeDimensions { nz, .. } => *nz,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        match self {
            Dimensions::TwoDimensions { nx, ny } => (*nx, *ny, 1),
            Dimensions::ThreeDimensions { nx, ny, nz } => (*nx, *ny, *nz),
        }
    }

    pub fn is_2d(&self) -> bool {
        matches!(self, Dimensions::TwoDimensions { .. })
    }

    pub fn total_vertices(&self) -> u32 {
        let (nx, ny, nz) = self.get_dimensions();
        nx * ny * nz
    }
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

    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn find_vertex(&self, id: u32) -> Result<&Vertex<T>, GeometryError> {
        self.vertices
            .iter()
            .find(|v| v.id == id)
            .ok_or(GeometryError::VertexNotFound { vertex_id: id })
    }

    pub fn is_boundary_vertex(&self, id: u32, direction: Direction) -> Result<bool, GeometryError> {
        let current_vertex: &Vertex<T> = self.find_vertex(id)?;
        
        // get dimensions based on mesh type 
        if self.dimensions.is_2d() && matches!(direction, Direction::Up | Direction::Down) {
            return Err(GeometryError::InvalidDirection { direction });
        }
        let (nx, ny, nz) = self.dimensions.get_dimensions();

        match direction {
            Direction::North => {
                match self.write_order {
                    WriteOrder::IJK => 
                    WriteOrder::JIK => 
                }
            }
            Direction::South => {

            }
            Direction::East => {

            }
            Direction::West => {

            }
            Direction::West => {

            }
            Direction::Up => {

            }
            Direction::Down => {

            }
        }
    }

    pub fn find_adjacent_vertex(&self, id: i32, direction: Direction) -> Result<&Vertex<T>, GeometryError> {
        // check if the given vertex exists 
        let current_vertex: &Vertex<T> = self.find_vertex(id)?;

        // get dimensions based on mesh type 
        if self.dimensions.is_2d() && matches!(direction, Direction::Up | Direction::Down) {
            return Err(GeometryError::InvalidDirection { direction });
        }
        let (nx, ny, nz) = self.dimensions.get_dimensions();

        // calculate the adjacent vertex id based on given direction
        let adjacent_id: i32 = match direction {
            Direction::North => {
                // check if we are at the northern boundary
                if current_vertex.id % self.ny == self.ny - 1 {
                    return None;
                }
                id + 1
            }
            Direction::South => {
                // check if we are at the southern boundary
                if current_vertex.id % self.ny == 0 {
                    return None;
                }
                id - 1
            }
            Direction::East => {
                // check if we are at the eastern boundary
                if current_vertex.id >= (self.nx - 1) * self.ny {
                    return None;
                }
                id + self.ny
            }
            Direction::West => {
                // check if we are at the western boundary
                if current_vertex.id < self.ny {
                    return None;
                }
                id - self.ny
            }
            Direction::Up => {
                // check if we are at the top boundary
                if current_vertex.id
            }
            Direction::Down => {

            }
        };
        Ok(self.find_vertex(adjacent_id)?)
    }
}
