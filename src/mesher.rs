#![allow(dead_code)]

use num_traits::Float;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use crate::geometry::*;

pub enum WallDistribution {
    Uniform, 
    HyperbolicTangent, 
}

pub struct Mesh<T: Float + Display> {
    pub vertices: Vec<Vertex<T>>,
    pub lines: Vec<Line<T>>,
}

impl<T: Float + Display> Mesh<T> {
    pub fn new() -> Mesh<T> {
        Mesh {
            vertices: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn draw(&self) {
        // TODO: implement this method
    }

    pub fn vertex_dump(&self, file_path: Option<&str>) {
        let path = file_path.unwrap_or("vertex_dump.txt");
        let mut file = File::create(path).expect("could not create file");
        
        for vertex in &self.vertices {
            // write to text file
            writeln!(file, "vertex id: {}, x: {}, y: {}, z: {}", vertex.id, vertex.coords.x, vertex.coords.y, vertex.coords.z)
                .expect("could not write to file");
        }
    }

    pub fn create_mesh_2d(
        &self,
        nx: i32, // number of points in x
        ny: i32, // number of points in y
        lenx: T,
        wall_distribution: WallDistribution,
        inlet_contour: impl Fn(T) -> T,
    ) {
        // calculate delta x (delta y will vary depending on the inlet contour function and the wall distribution)
        let dx: T = lenx / (T::from(nx).unwrap() - T::one());

        // create starting coordinates
        let mut x: T = T::zero();
        let mut y: T = T::zero();

        // create empty vector to store vertices
        let mut vertices: Vec<Vertex<T>> = Vec::new();
        let mut vertex_id: i32 = 0;

        match wall_distribution {
            WallDistribution::Uniform => {
                for _ in 0..nx {
                    // calculate domain height at given x and find corresponding dy
                    let leny: T = inlet_contour(x);
                    let dy: T = leny / (T::from(ny).unwrap() - T::one());
                    
                    for _ in 0..ny {
                        // create vertex at current point (x, y) and push to vertex vec
                        let vertex: Vertex<T> = Vertex::new(vertex_id, x, y, T::one());
                        vertices.push(vertex);

                        // increment vertex id and step by dy
                        vertex_id += 1;
                        y = y + dy;
                    }
                    // step by dx
                    x = x + dx;
                }
            }
            WallDistribution::HyperbolicTangent => {
                // todo
            } 
        }
    }
}