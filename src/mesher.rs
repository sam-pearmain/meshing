use num_traits::Float;
use crate::geometry::*;

enum WallDistribution {
    Uniform, 
    HyperbolicTangent, 
}

fn create_mesh<T: Float>(
    nx: i32, // number of points in x
    ny: i32, // number of points in y
    domain_length: T,
    wall_distribution: WallDistribution,
    inlet_contour: impl Fn(T) -> T,
) {
    // calculate delta x (delta y will vary depending on the inlet contour function and the wall distribution)
    let dx: T = domain_length / T::from(nx).unwrap();

    // create starting coordinates
    let mut x: T = T::zero();
    let mut y: T = T::zero();

    // create empty vector to store vertices
    let vertices: Vec<Vertex<T>> = Vec::new();

    
}