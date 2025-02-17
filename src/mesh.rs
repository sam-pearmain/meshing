#![allow(dead_code)]

use num_traits::Float;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use crate::utils::error::*;

pub enum WallDistribution {
    Uniform, 
    HyperbolicTangent, 
    TopClusteredTangent,
}

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Mesh<'a, T: Float + Display + Into<f64> + Copy> {
    pub vertices: Vec<Vertex<T>>, // maybe change this to its own Vertices struct so its methods are cleaner
    pub nodes: Vec<Node2D<'a, T>>,
    pub nx: i32,
    pub ny: i32,
}

impl<'a, T: Float + Display + Into<f64> + Copy> Mesh<'a, T> {
    pub fn new() -> Mesh<'a, T> {
        Mesh {
            vertices: Vec::new(),
            nodes: Vec::new(),
            nx: 0,
            ny: 0, 
        }
    }

    pub fn find_vertex(&self, id: i32) -> Option<&Vertex<T>> {
        self.vertices
            .iter()
            .find(|&v| v.id == id)
    }

    pub fn find_adjacent_vertex(&self, id: i32, direction: Direction) -> Option<&Vertex<T>> {
        // check if the given vertex exists 
        let current_vertex: &Vertex<T> = self.find_vertex(id)?;

        // calculate the adjacent vertex id based on given direction
        let adjacent_id: i32 = match direction {
            Direction::North => {
                // check if we are at the top boundary
                if current_vertex.id % self.ny == self.ny - 1 {
                    return None;
                }
                id + 1
            }
            Direction::South => {
                // check if we are at the bottom boundary
                if current_vertex.id % self.ny == 0 {
                    return None;
                }
                id - 1
            }
            Direction::East => {
                // check if we are at the right boundary
                if current_vertex.id >= (self.nx - 1) * self.ny {
                    return None;
                }
                id + self.ny
            }
            Direction::West => {
                // check if we are at the left boundary
                if current_vertex.id < self.ny {
                    return None;
                }
                id - self.ny
            }
        };
        self.find_vertex(adjacent_id)
    }

    pub fn get_vertices(&self) -> (Vec<T>, Vec<T>, Vec<T>) {
        let mut x: Vec<T> = Vec::new();
        let mut y: Vec<T> = Vec::new();
        let mut z: Vec<T> = Vec::new();

        for vertex in &self.vertices {
            x.push(vertex.coords.x);
            y.push(vertex.coords.y);
            z.push(vertex.coords.z);
        }

        (x, y, z)
    }

    pub fn draw_mesh(&self, filename: &str) {
        let (x, y, _) = self.get_vertices();
        crate::utils::plotting::simple_scatter_plot(&x, &y, "mesh", filename).unwrap();
    }

    pub fn vertex_dump(&self, file_path: Option<&str>) -> std::io::Result<()> {
        let path = file_path.unwrap_or("vertex-dump.txt");
        let mut file = File::create(path)?;
        
        for vertex in &self.vertices {
            // write to text file
            writeln!(file, "vertex id: {}, x: {:.4}, y: {:.4}, z: {:.4}",
                vertex.id, vertex.coords.x, vertex.coords.y, vertex.coords.z
            )?;   
        }
        Ok(())
    }

    pub fn create_mesh_2d(
        &mut self,
        nx: i32, // number of points in x
        ny: i32, // number of points in y
        lenx: T,
        wall_distribution: WallDistribution,
        inlet_contour: impl Fn(T) -> T,
        beta: Option<T>, // controls point clustering (higher = more clustering)
    ) {
        // calculate delta x (delta y will vary depending on the inlet contour function and the wall distribution)
        let dx: T = lenx / (T::from(nx).unwrap() - T::one());

        // create starting coordinates
        let mut x: T = T::zero();
        let mut y: T = T::zero();

        // create empty vector to store vertices
        let mut vertex_id: i32 = 0;

        match wall_distribution {
            WallDistribution::Uniform => {
                for _ in 0..nx {
                    // calculate domain height at given x and find corresponding dy
                    let leny: T = inlet_contour(x);
                    let dy: T = leny / (T::from(ny).unwrap() - T::one());
                    
                    for _ in 0..ny {
                        // create vertex at current point (x, y) and push to vertices
                        let vertex: Vertex<T> = Vertex::new(vertex_id, x, y, T::one());
                        self.vertices.push(vertex);

                        // increment vertex id and step by dy
                        vertex_id += 1;
                        y = y + dy;
                    }
                    // reset y and step by dx
                    y = T::zero();
                    x = x + dx;
                }
            }
            WallDistribution::HyperbolicTangent => {
                let beta: T = beta.unwrap_or(T::from(2.0).unwrap());

                for _ in 0..nx {
                    // calculate domain height at current x position
                    let leny: T = inlet_contour(x);

                    for j in 0..ny {
                        // calculate normalised coordinate eta between 0 and 1
                        let eta: T = T::from(j).unwrap() / T::from(ny - 1).unwrap();

                        // apply hyperbolic tangent distribution
                        // y(eta) = leny * (1 + tanh(beta * (eta - 0.5)) / tanh(beta / 2))
                        let tanh_term = 
                            (beta * (eta - T::from(0.5).unwrap())). tanh() / 
                            (beta * T::from(0.5).unwrap()).tanh();
                        let y = leny * (T::one() + tanh_term);

                        // create vertex at current point (x, y)
                        let vertex: Vertex<T> = Vertex::new(vertex_id, x, y, T::one());
                        self.vertices.push(vertex);

                        // increment vertex id
                        vertex_id += 1;
                    }
                    // step in x direction
                    x = x + dx;
                }
            } 
            WallDistribution::TopClusteredTangent => {
                let beta: T = beta.unwrap_or(T::from(2.0).unwrap());

                for _ in 0..nx {
                    // calculate domain height at current x position
                    let leny: T = inlet_contour(x);

                    for j in 0..ny {
                        // calculate normalised coordinate eta between 0 and 1
                        let eta: T = T::from(j).unwrap() / T::from(ny - 1).unwrap();
                        
                        // top-boundary-clustered distribution
                        let tanh_term = (beta * eta).tanh() / beta.tanh();
                        y = leny * tanh_term;

                        // create vertex at current (x, y) and push to vertices
                        let vertex: Vertex<T> = Vertex::new(vertex_id, x, y, T::one());
                        self.vertices.push(vertex);

                        // increment vertex id
                        vertex_id += 1;
                    }
                    // step in x direction
                    x = x + dx;
                }
            }
        }
    }

    pub fn export_stl(&self, filename: &str) {
        // todo
    }

    fn populate_nodes(&self) -> Result<(), MeshError<T>> {
        if self.is_empty() {
            return Err(MeshError::EmptyMesh)
        }

        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}