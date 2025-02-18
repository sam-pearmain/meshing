#![allow(dead_code)]

use crate::utils::error::*;
use super::prelude::*;
use super::Point3D;

// TODO: need to have more standardised errors and error cascading between functions

#[derive(Debug)]
pub struct Vertex<T: Float> {
    pub id: u32,
    pub coords: Point3D<T>,
}

impl<T: Float + fmt::Display> Vertex<T> {
    pub fn new(id: u32, x: T, y: T, z: T) -> Result<Vertex<T>, GeometryError<T>> {
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

#[derive(Debug)]
pub enum Dimensions {
    TwoDimensions{ nx: u32, ny: u32 }, 
    ThreeDimensions{ nx: u32, ny: u32, nz: u32 },
}

impl Dimensions {
    pub fn is_2d(&self) -> bool {
        matches!(self, Dimensions::TwoDimensions { .. })
    }

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

    pub fn set_nx(&mut self, new_nx: u32) {
        match self {
            Dimensions::TwoDimensions { nx, .. } => *nx = new_nx,
            Dimensions::ThreeDimensions { nx, .. } => *nx = new_nx,
        }
    }

    pub fn set_ny(&mut self, new_ny: u32) {
        match self {
            Dimensions::TwoDimensions { ny, .. } => *ny = new_ny,
            Dimensions::ThreeDimensions { ny, .. } => *ny = new_ny,
        }
    }

    pub fn set_nz(&mut self, new_nz: u32) -> Result<(), &'static str>{
        match self {
            Dimensions::TwoDimensions { .. } => Err("cannot set nz for 2D dimensions"),
            Dimensions::ThreeDimensions { nz, .. } => {
                *nz = new_nz;
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn total_vertices(&self) -> Result<u32, MeshError<T>> {
        if self.is_empty() {
            return Err(MeshError::EmptyMesh);
        }
        let (nx, ny, nz) = self.dimensions.get_dimensions();
        Ok(nx * ny * nz)
    }

    pub fn get_final_vertex(&self) -> Result<&Vertex<T>, MeshError<T>> {
        self.vertices
            .last()
            .ok_or(MeshError::EmptyMesh)
    }

    pub fn add_vertex(&mut self, x: T, y: T, z: T) -> Result<(), GeometryError<T>> {
        if self.dimensions.is_2d() && z != T::one() {
            return Err(GeometryError::InvalidVertexCoordinate { coordinate: 'z', expected: T::one(), received: z })
        }
        
        let next_id = if self.is_empty() {
            1
        } else {
            self.get_final_vertex()
                .map_err(|_| GeometryError::VertexNotFound { vertex_id: 1 })?
                .id + 1
        };

        let total_vertices = self.total_vertices()
            .map_err(|_| GeometryError::InvalidVertexID)?;
        if next_id > total_vertices {
            return Err(GeometryError::VertexLimitExceeded{ 
                limit: total_vertices,
                attempted: next_id,
            });
        }

        let vertex = Vertex::new(next_id, x, y, z)?;
        self.vertices.push(vertex);
        Ok(())
    }

    pub fn vertex_exists(&self, id: u32) -> bool {
        self.vertices
            .iter()
            .any(|v| v.id == id)
    }

    pub fn find_vertex(&self, id: u32) -> Result<&Vertex<T>, GeometryError<T>> {
        self.vertices
            .iter()
            .find(|v| v.id == id)
            .ok_or(GeometryError::VertexNotFound { vertex_id: id })
    }

    pub fn is_boundary_vertex(&self, id: u32, direction: Direction) -> Result<bool, GeometryError<T>> {
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

    pub fn find_adjacent_vertex(&self, id: u32, direction: Direction) -> Result<&Vertex<T>, GeometryError<T>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_2d_collection() -> VertexCollection<f64> {
        let dimensions = Dimensions::TwoDimensions { nx: 3, ny: 3 };
        let mut collection = VertexCollection::new(dimensions, WriteOrder::IJK);
        
        // create a 3x3 grid of vertices
        for i in 1..=9 {
            collection.add_vertex((i - 1) as f64 % 3.0, (i - 1) as f64 / 3.0, 1.0).expect("idk");
        }
        collection
    }

    #[test]
    fn test_vertex_exists() {
        let collection = create_2d_collection();
        assert!(collection.vertex_exists(1));
        assert!(collection.vertex_exists(9));
        assert!(!collection.vertex_exists(0));
        assert!(!collection.vertex_exists(10));
    }

    #[test]
    fn test_find_vertex() {
        let collection = create_2d_collection();
        assert!(collection.find_vertex(1).is_ok());
        assert!(collection.find_vertex(0).is_err());
    }

    #[test]
    fn test_is_boundary_vertex_2d_ijk() {
        let collection = create_2d_collection();
        
        // test corners
        assert!(collection.is_boundary_vertex(1, Direction::South).unwrap()); // bottom-left
        assert!(collection.is_boundary_vertex(3, Direction::South).unwrap()); // bottom-right
        assert!(collection.is_boundary_vertex(7, Direction::North).unwrap()); // top-left
        assert!(collection.is_boundary_vertex(9, Direction::North).unwrap()); // top-right

        // test edges
        assert!(collection.is_boundary_vertex(2, Direction::South).unwrap()); // bottom edge
        assert!(collection.is_boundary_vertex(4, Direction::West).unwrap());  // left edge
        assert!(collection.is_boundary_vertex(6, Direction::East).unwrap());  // right edge
        assert!(collection.is_boundary_vertex(8, Direction::North).unwrap()); // top edge

        // test center
        assert!(!collection.is_boundary_vertex(5, Direction::North).unwrap());
        assert!(!collection.is_boundary_vertex(5, Direction::South).unwrap());
        assert!(!collection.is_boundary_vertex(5, Direction::East).unwrap());
        assert!(!collection.is_boundary_vertex(5, Direction::West).unwrap());
    }

    #[test]
    fn test_find_adjacent_vertex_2d_ijk() {
        let collection = create_2d_collection();
        
        // check all directions from centre
        assert_eq!(collection.find_adjacent_vertex(5, Direction::North).unwrap().id, 8);
        assert_eq!(collection.find_adjacent_vertex(5, Direction::South).unwrap().id, 2);
        assert_eq!(collection.find_adjacent_vertex(5, Direction::East).unwrap().id, 6);
        assert_eq!(collection.find_adjacent_vertex(5, Direction::West).unwrap().id, 4);
    }

    #[test]
    fn test_boundary_errors() {
        let collection = create_2d_collection();
        
        // test boundary errors
        assert!(matches!(
            collection.find_adjacent_vertex(1, Direction::South),
            Err(GeometryError::BoundaryVertex { vertex_id: 1, direction: Direction::South })
        ));

        assert!(matches!(
            collection.find_adjacent_vertex(9, Direction::North),
            Err(GeometryError::BoundaryVertex { vertex_id: 9, direction: Direction::North })
        ));
    }

    #[test]
    fn test_invalid_vertex() {
        let collection = create_2d_collection();
        
        assert!(matches!(
            collection.find_adjacent_vertex(0, Direction::North),
            Err(GeometryError::VertexNotFound { vertex_id: 0 })
        ));

        assert!(matches!(
            collection.find_adjacent_vertex(10, Direction::South),
            Err(GeometryError::VertexNotFound { vertex_id: 10 })
        ));
    }

    #[test]
    fn test_invalid_direction_2d() {
        let collection = create_2d_collection();
        
        assert!(matches!(
            collection.find_adjacent_vertex(5, Direction::Up),
            Err(GeometryError::InvalidDirection { direction: Direction::Up })
        ));

        assert!(matches!(
            collection.find_adjacent_vertex(5, Direction::Down),
            Err(GeometryError::InvalidDirection { direction: Direction::Down })
        ));
    }

    #[test]
    fn test_invalid_z_coordinate_2d() {
        let dimensions = Dimensions::TwoDimensions { nx: 3, ny: 3 };
        let mut collection = VertexCollection::new(dimensions, WriteOrder::IJK);
        
        assert!(matches!(
            collection.add_vertex(0.0, 0.0, 2.0),
            Err(GeometryError::InvalidVertexCoordinate { 
                coordinate: 'z', 
                expected: 1.0, 
                received: 2.0,
            })
        ));
    }
}