use super::{prelude::*, Dimensioned, Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D<T: Float> {
    x: T,
    y: T,
}

impl<T: Float> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2D::from_coordinates((x, y))
    }
}

impl<T: Float> Dimensioned for Point2D<T> {
    fn is_2d(&self) -> bool {
        true
    }
}

impl<F: Float> Point<F> for Point2D<F> {
    type Coordinates = (F, F);
    
    fn origin() -> Self {
        Point2D { x: F::zero(), y: F::zero() }
    }

    fn from_coordinates(coords: Self::Coordinates) -> Self {
        Point2D { x: coords.0, y: coords.1 }
    }

    fn coordinates(&self) -> Self::Coordinates {
        (self.x, self.y)
    }
}

impl<T: Float + fmt::Display> fmt::Display for Point2D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D ({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3D::from_coordinates((x, y, z))
    }
}

impl<T: Float> Dimensioned for Point3D<T> {
    fn is_2d(&self) -> bool {
        false
    }
}

impl<F: Float> Point<F> for Point3D<F> {
    type Coordinates = (F, F, F);

    fn origin() -> Self {
        Point3D { x: F::zero(), y: F::zero(), z: F::zero() }
    }

    fn from_coordinates(coords: Self::Coordinates) -> Self {
        Point3D { x: coords.0, y: coords.1, z: coords.2 }
    }

    fn coordinates(&self) -> Self::Coordinates {
        (self.x, self.y, self.z)
    }
}

impl<T: Float + fmt::Display> fmt::Display for Point3D<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point3D ({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_creation() {
        let p1 = Point2D::new(1.0, 2.0);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 2.0);

        let p2 = Point2D::from_coordinates((3.0, 4.0));
        assert_eq!(p2.x, 3.0);
        assert_eq!(p2.y, 4.0);

        let origin = Point2D::<f64>::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
    }

    #[test]
    fn test_point2d_dimensioned() {
        let p = Point2D::new(1.0, 2.0);
        assert!(p.is_2d());
        assert_eq!(p.dimensions(), 2);
    }

    #[test]
    fn test_point2d_coordinates() {
        let p = Point2D::new(1.0, 2.0);
        assert_eq!(p.coordinates(), (1.0, 2.0));
    }

    #[test]
    fn test_point2d_display() {
        let p = Point2D::new(1.0, 2.0);
        assert_eq!(format!("{}", p), "Point2D (1, 2)");
    }

    #[test]
    fn test_point3d_creation() {
        let p1 = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p1.x, 1.0);
        assert_eq!(p1.y, 2.0);
        assert_eq!(p1.z, 3.0);

        let p2 = Point3D::from_coordinates((4.0, 5.0, 6.0));
        assert_eq!(p2.x, 4.0);
        assert_eq!(p2.y, 5.0);
        assert_eq!(p2.z, 6.0);

        let origin = Point3D::<f64>::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);
    }

    #[test]
    fn test_point3d_dimensioned() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert!(!p.is_2d());
        assert_eq!(p.dimensions(), 3);
    }

    #[test]
    fn test_point3d_coordinates() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.coordinates(), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_point3d_display() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(format!("{}", p), "Point3D(1, 2, 3)");
    }

    #[test]
    fn test_point_equality() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = Point2D::new(1.0, 2.0);
        let p3 = Point2D::new(2.0, 1.0);
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        let p4 = Point3D::new(1.0, 2.0, 3.0);
        let p5 = Point3D::new(1.0, 2.0, 3.0);
        let p6 = Point3D::new(3.0, 2.0, 1.0);
        assert_eq!(p4, p5);
        assert_ne!(p4, p6);
    }

    #[test]
    fn test_point_clone() {
        let p1 = Point2D::new(1.0, 2.0);
        let p2 = p1.clone();
        assert_eq!(p1, p2);

        let p3 = Point3D::new(1.0, 2.0, 3.0);
        let p4 = p3.clone();
        assert_eq!(p3, p4);
    }
}