use super::prelude::*;
use super::{Point3D, Line};

pub struct Node2D<'a, T: Float> {
    pub id: i32,
    pub north_face: Line<'a, T>,
    pub east_face: Line<'a, T>,
    pub south_face: Line<'a, T>,
    pub west_face: Line<'a, T>,
}

impl<'a, T: Float> Node2D<'a, T> {
    pub fn new(
        id: i32, 
        north: Line<'a, T>, 
        east: Line<'a, T>, 
        south: Line<'a, T>, 
        west: Line<'a, T>
    ) -> Node2D<'a, T> {
        Node2D { 
            id, 
            north_face: north,
            east_face: east, 
            south_face: south,
            west_face: west,
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
