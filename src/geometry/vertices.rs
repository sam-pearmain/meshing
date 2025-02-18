use super::point::{Point2D, Point3D};
use super::prelude::*;
use super::{Dimensioned, Point};

pub struct Vertex<F: Float, P: Point<F>> {
    id: usize,
    coords: P,
}

impl<F: Float, P: Point<F>> Vertex<F, P> {
    fn new_2d(id: usize, x: F, y: F) -> Self {
        Vertex { id, coords: Point2D::new(x, y)}
    }

    fn new_3d(id: usize, x: F, y: F, z: F) -> Self {
        Vertex { id, coords: Point3D::new(x, y, z) }
    }

    fn is_2d(&self) -> bool {
        self.coords.is_2d()
    }
}

pub enum Dimensions {
    Two{ nx: usize, ny: usize }, 
    Three{ nx: usize, ny: usize, nz: usize },
}

impl Dimensioned for Dimensions {
    fn is_2d(&self) -> bool {
        matches!(self, Dimensions::Two { .. })
    }
}

impl Dimensions {
    fn get_nx(&self) -> usize {
        match self {
            Dimensions::Two { nx, .. } => *nx,
            Dimensions::Three { nx, .. } => *nx,
        }
    }

    fn get_ny(&self) -> usize {
        match self {
            Dimensions::Two { ny, .. } => *ny,
            Dimensions::Three { ny, .. } => *ny,
        }
    }

    fn get_nz(&self) -> usize {
        match self {
            Dimensions::Two { .. } => 1_usize,
            Dimensions::Three { nz, .. } => *nz,
        }
    }
}

pub enum WriteOrder {
    XYZ,
    YXZ,
}

pub struct VertexCollection<F: Float> {
    vertices: Vec<Vertex<F>>,
    dimensions: Dimensions,
    write_order: WriteOrder,
}