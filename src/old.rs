use std::fmt::Display;
use num_traits::Float;

#[derive(Debug)]
pub struct Point3D<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D { x, y, z }
    }

    pub fn as_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

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
 
pub struct Node2D<'a, T: Float> {
    pub id: i32,
    pub north_face: Line<'a, T>,
    pub east_face: Line<'a, T>,
    pub south_face: Line<'a, T>,
    pub west_face: Line<'a, T>,
}

impl<'a, T: Float> Node2D<'a, T> {
    pub fn new(id: i32, ln: Line<'a, T>, le: Line<'a, T>, ls: Line<'a, T>, lw: Line<'a, T>) -> Node2D<'a, T> {
        Node2D { 
            id, 
            north_face: ln,
            east_face: le, 
            south_face: ls,
            west_face: lw,
        }
    }

    pub fn find_centre(&self) -> Point3D<T> {
        let four: T = T::from(4).unwrap();
        let x_average: T = (
            self.north_face.start.coords.x + 
            self.north_face.end.coords.x + 
            self.south_face.start.coords.x + 
            self.south_face.end.coords.x 
        ) / four;
        let y_average: T = (
            self.north_face.start.coords.y + 
            self.north_face.end.coords.y + 
            self.south_face.start.coords.y + 
            self.south_face.end.coords.y
        ) / four;
        Point3D {
            x: x_average,
            y: y_average,
            z: T::one(),
        }
    }

    pub fn check_orthogonality(&self) {
        // check if the interior diagonals are perpendicular
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex {
            id: 1,
            coords: Point3D { x: 1.0, y: 2.0, z: 1.0 },
        };

        assert_eq!(vertex.id, 1);
        assert_eq!(vertex.coords.x, 1.0);
        assert_eq!(vertex.coords.y, 2.0);
        assert_eq!(vertex.coords.z, 1.0);
    }

    #[test]
    fn test_find_node_centre() {
        let v1 = Vertex::new(1, 0.0, 0.0, 1.0);
        let v2 = Vertex::new(2, 1.0, 0.0, 1.0);
        let v3 = Vertex::new(3, 1.0, 1.0, 1.0);
        let v4 = Vertex::new(4, 0.0, 1.0, 1.0);

        let north = Line::new(&v3, &v4);  // top edge
        let east = Line::new(&v2, &v3);   // right edge
        let south = Line::new(&v1, &v2);  // bottom edge
        let west = Line::new(&v4, &v1);   // left edge

        let node: Node2D<f64> = Node2D::new(
            1,
            north, 
            east, 
            south, 
            west,
        );

        let centre: Point3D<f64> = node.find_centre();
        assert_eq!(centre.x, 0.5);
        assert_eq!(centre.y, 0.5);
        assert_eq!(centre.z, 1.0);
    }
}