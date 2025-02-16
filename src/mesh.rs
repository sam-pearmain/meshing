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

pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Mesh<T: Float + Display + Into<f64> + Copy> {
    pub vertices: Vec<Vertex<T>>,
    pub nodes: Vec<Node2D<T>>,
    pub nx: i32,
    pub ny: i32,
}

impl<T: Float + Display + Into<f64> + Copy> Mesh<T> {
    pub fn new() -> Mesh<T> {
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

    pub fn get_adjacent_vertex(&self, id: i32, direction: Direction) -> Option<&Vertex<T>> {
        // check if the given vertex exists 
        let current_vertex = self.find_vertex(id).expect("vertex does not exist");

        // calculate the adjacent vertex id based on given direction
        let adjacent_id = match direction {
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

    pub fn draw_mesh(&self) {
        let raw_vertices = self.get_vertices();
        let x: Vec<T> = raw_vertices.0;
        let y: Vec<T> = raw_vertices.1;

        crate::utils::plotting::simple_scatter_plot(&x, &y, "mesh", "mesh.png").unwrap();
    }

    pub fn vertex_dump(&self, file_path: Option<&str>) {
        let path = file_path.unwrap_or("vertex-dump.txt");
        let mut file = File::create(path).expect("could not create file");
        
        for vertex in &self.vertices {
            // write to text file
            writeln!(file, "vertex id: {}, x: {:.4}, y: {:.4}, z: {:.4}", vertex.id, vertex.coords.x, vertex.coords.y, vertex.coords.z)
                .expect("could not write to file");
        }
    }

    pub fn create_mesh_2d(
        &mut self,
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
                // todo
            } 
        }
    }
}