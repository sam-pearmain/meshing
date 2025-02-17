use super::prelude::*;

#[derive(Debug)]
pub struct Point3D<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3D { x, y, z }
    }

    pub fn origin() -> Self {
        Point3D {
            x: T::zero(), 
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn distance_to(&self, other: &Point3D<T>) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }

    pub fn as_tuple(&self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }
}

impl<T: Float + Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_origin() {
        let p = Point3D::<f64>::origin();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn test_distance() {
        let p1 = Point3D::new(0.0, 0.0, 0.0);
        let p2 = Point3D::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }
}