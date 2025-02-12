use num_traits::Float;

pub struct Cartesian2D<T: Float> {
    pub x: T,
    pub y: T,
}

pub struct Vertex2D<T: Float> {
    pub id: i32,
    pub coords: Cartesian2D<T>,
}

pub struct Line2D<T: Float> {
    pub id: i32,
    pub start: Vertex2D<T>,
    pub end: Vertex2D<T>,
}

pub struct Node2D<T: Float> {
    pub id: i32,
    pub north_face: Line2D<T>,
    pub east_face: Line2D<T>,
    pub south_face: Line2D<T>,
    pub west_face: Line2D<T>,
}

impl<T: Float> Node2D<T> {
    pub fn find_centre(&self) -> Cartesian2D<T> {
        let x: T = self.north_face.start.coords.x + self.north_face.end.coords.x + self.south_face.start.coords.x + self.south_face.end.coords.x / 4.0 as T;
        let y: T = self.east_face.start.coords.y + self.east_face.end.coords.y + self.west_face.start.coords.y + self.west_face.end.coords.y / 4 as T;
        Cartesian2D {
            x: x,
            y: y,
        }
    }
}