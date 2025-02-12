use num_traits::Float;

pub struct Cartesian2D<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Cartesian2D<T> {
    fn new(x: T, y: T) -> Cartesian2D<T> {
        Cartesian2D { x, y }
    }
}

pub struct Vertex2D<T: Float> {
    pub id: i32,
    pub coords: Cartesian2D<T>,
}

impl<T: Float> Vertex2D<T> {
    pub fn new(id: i32, x: T, y: T) -> Vertex2D<T> {
        Vertex2D {
            id: id, 
            coords: Cartesian2D::new(x, y),
        }
    }
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
        let four: T = T::from(4).unwrap();
        let x: T = self.north_face.start.coords.x + 
                   self.north_face.end.coords.x + 
                   self.south_face.start.coords.x + 
                   self.south_face.end.coords.x 
                   / four;
        let y: T = self.north_face.start.coords.y + 
                   self.north_face.end.coords.y + 
                   self.south_face.start.coords.y + 
                   self.south_face.end.coords.y 
                   / four;
        Cartesian2D {
            x: x,
            y: y,
        }
    }
}