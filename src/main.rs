use num_traits::Float;

mod geometry;
mod mesh;
mod utils;

fn main() {
    let mut mesh = mesh::Mesh::new();

    mesh.create_mesh_2d(
        200, 
        100, 
        2.0_f64, 
        mesh::WallDistribution::Uniform, 
        inlet_contour, 
        None,
    );

    mesh.vertex_dump(None).expect("failed to dump vertices");
    mesh.draw_mesh("mesh-uniform.png");

    let mut mesh_2 = mesh::Mesh::new();

    mesh_2.create_mesh_2d(
        200, 
        100, 
        2.0_f64, 
        mesh::WallDistribution::HyperbolicTangent, 
        inlet_contour, 
        None
    );

    mesh_2.draw_mesh("mesh-tanh.png");
}

fn inlet_contour<T: Float>(x: T) -> T {
    T::one() - (T::one() / T::from(10.0).unwrap() * x.powi(2))
}