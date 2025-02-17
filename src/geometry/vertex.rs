#![allow(dead_code)]

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

impl fmt::Display for Direction {
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

#[derive(Debug, Clone, Copy)]
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

    pub fn vertex_exists(&self, id: u32) -> bool {
        self.vertices
            .iter()
            .any(|v| v.id == id)
    }

    pub fn find_vertex(&self, id: u32) -> Result<&Vertex<T>, GeometryError> {
        self.vertices
            .iter()
            .find(|v| v.id == id)
            .ok_or(GeometryError::VertexNotFound { vertex_id: id })
    }

    pub fn is_boundary_vertex(&self, id: u32, direction: Direction) -> Result<bool, GeometryError> {
        // get dimensions based on mesh type 
        if self.dimensions.is_2d() && matches!(direction, Direction::Up | Direction::Down) {
            return Err(GeometryError::InvalidDirection { direction });
        }
        let (nx, ny, nz) = self.dimensions.get_dimensions();

        match (direction, self.write_order) {
            // IJK order: x -> y -> z
            (Direction::South, WriteOrder::IJK) => Ok(id <= nx),            // first nx vertices
            (Direction::North, WriteOrder::IJK) => Ok(id > nx * (ny - 1)),  // last nx vertices in plane
            (Direction::West, WriteOrder::IJK) => Ok((id - 1) % nx == 0),   // first vertex in each row
            (Direction::East, WriteOrder::IJK) => Ok(id % nx == 0),         // last vertex in each row
            
            // JIK order: y -> x -> z
            (Direction::South, WriteOrder::JIK) => Ok(id % ny == 1),        // first vertex in each column
            (Direction::North, WriteOrder::JIK) => Ok(id % ny == 0),        // last vertex in each column
            (Direction::West, WriteOrder::JIK) => Ok(id <= ny),             // first ny vertices
            (Direction::East, WriteOrder::JIK) => Ok(id > ny * (nx - 1)),   // last ny vertices in plane
            
            // 3D boundaries
            (Direction::Up, _) => Ok(id > nx * ny * (nz - 1)),              // top layer
            (Direction::Down, _) => Ok(id <= nx * ny),                      // bottom layer
        }
    }

    pub fn find_adjacent_vertex(&self, id: u32, direction: Direction) -> Result<&Vertex<T>, GeometryError> {
        // check if requested vertex exists
        if !self.vertex_exists(id) {
            return Err(GeometryError::VertexNotFound { vertex_id: id });
        }

        // get dimensions based on mesh type 
        if self.dimensions.is_2d() && matches!(direction, Direction::Up | Direction::Down) {
            return Err(GeometryError::InvalidDirection { direction });
        }
        let (nx, ny, _) = self.dimensions.get_dimensions();

        // check if we are at a boundary in the requested direction
        if self.is_boundary_vertex(id, direction)? {
            return Err(GeometryError::BoundaryVertex { vertex_id: id, direction: direction });
        }

        // calculate the adjacent vertex's id
        let adjacent_id: u32 = match (direction, self.write_order) {
            // IJK order: x -> y -> z
            (Direction::North, WriteOrder::IJK) => id + nx, // move up one row
            (Direction::South, WriteOrder::IJK) => id - nx, // move down one row
            (Direction::East, WriteOrder::IJK) => id + 1,   // move right
            (Direction::West, WriteOrder::IJK) => id - 1,   // move left
            
            // JIK order: y -> x -> z
            (Direction::North, WriteOrder::JIK) => id + 1,  // move up
            (Direction::South, WriteOrder::JIK) => id - 1,  // move down
            (Direction::East, WriteOrder::JIK) => id + ny,  // move right one column
            (Direction::West, WriteOrder::JIK) => id - ny,  // move left one column
            
            // 3D movements (same for both orders)
            (Direction::Up, _) => id + (nx * ny),           // move up one layer
            (Direction::Down, _) => id - (nx * ny),         // move down one layer
        };

        Ok(self.find_vertex(adjacent_id)?)
    }
}
