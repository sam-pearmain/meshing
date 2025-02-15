use num_traits::Float;

mod geometry;
mod mesher;

fn main() {
    let mut mesh = mesher::Mesh::new();

    mesh.create_mesh_2d(
        200, 
        100, 
        2.0_f32, 
        mesher::WallDistribution::Uniform, 
        inlet_contour
    );

    mesh.vertex_dump(None);
}

fn inlet_contour<T: Float>(x: T) -> T {
    T::from(2.0).unwrap() * x + T::from(1.0).unwrap()
}