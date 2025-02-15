#![allow(dead_code)]

use num_traits::Float;

pub struct Cartesian<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Cartesian<T> {
    pub fn new(x: T, y: T) -> Cartesian<T> {
        Cartesian { x, y }
    }
}

pub struct Vertex<T: Float> {
    pub id: i32,
    pub coords: Cartesian<T>,
}

impl<T: Float> Vertex<T> {
    pub fn new(id: i32, x: T, y: T) -> Vertex<T> {
        Vertex {
            id: id, 
            coords: Cartesian::new(x, y),
        }
    }
}

pub struct Line<T: Float> {
    pub start: Cartesian<T>,
    pub end: Cartesian<T>,
}

impl<T: Float> Line<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Line<T> {
        Line {
            start: Cartesian::new(x1, y1),
            end: Cartesian::new(x2, y2),
        }
    }

    pub fn between_vertices(v1: Vertex<T>, v2: Vertex<T>) -> Line<T> {
        let point1 = Cartesian::new(v1.coords.x, v1.coords.y);
        let point2 = Cartesian::new(v2.coords.x, v2.coords.y);
        Line {
            start: point1,
            end: point2,
        }
    }

    pub fn gradient(&self) -> T {
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;
        y_diff / x_diff
    }

    pub fn length(&self) -> T {
        let x_diff = self.end.x - self.start.x;
        let y_diff = self.end.y - self.start.y;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }
}
 
pub struct Node<T: Float> {
    pub id: i32,
    pub north_face: Line<T>,
    pub east_face: Line<T>,
    pub south_face: Line<T>,
    pub west_face: Line<T>,
}

impl<T: Float> Node<T> {
    pub fn find_centre(&self) -> Cartesian<T> {
        let four: T = T::from(4).unwrap();
        let x_average: T = (
            self.north_face.start.x + 
            self.north_face.end.x + 
            self.south_face.start.x + 
            self.south_face.end.x 
        ) / four;
        let y_average: T = (
            self.north_face.start.y + 
            self.north_face.end.y + 
            self.south_face.start.y + 
            self.south_face.end.y
        ) / four;
        Cartesian {
            x: x_average,
            y: y_average,
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
            coords: Cartesian { x: 1.0, y: 2.0 }
        };

        assert_eq!(vertex.id, 1);
        assert_eq!(vertex.coords.x, 1.0);
        assert_eq!(vertex.coords.y, 2.0);
    }

    #[test]
    fn test_find_node_centre() {
        let node = Node {
            id: 1,
            north_face: Line::new(0.0, 1.0, 1.0, 1.0),
            east_face: Line::new(1.0, 1.0, 1.0, 0.0),
            south_face: Line::new(1.0, 0.0, 0.0, 0.0),
            west_face: Line::new(0.0, 0.0, 0.0, 1.0),
        };
        let centre = node.find_centre();
        assert_eq!(centre.x, 0.5);
        assert_eq!(centre.y, 0.5);
    }
}