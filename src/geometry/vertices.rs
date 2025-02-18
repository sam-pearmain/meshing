use super::point::{Point2D, Point3D};
use super::prelude::*;
use super::{Dimensioned, Point};

pub struct Vertex<F: Float, P: Point<F>> {
    id: usize,
    coords: P,
}

impl<F: Float, P: Point<F>> Vertex<F, P> {
    pub fn is_2d(&self) -> bool {
        self.coords.is_2d()
    }
}

impl<F: Float> Vertex<F, Point2D<F>> {
    pub fn new_2d(id: usize, x: F, y: F) -> Self {
        Vertex { id, coords: Point2D::new(x, y)}
    }
}

impl<F: Float> Vertex<F, Point3D<F>> {
    pub fn new_3d(id: usize, x: F, y: F, z: F) -> Self {
        Vertex { id, coords: Point3D::new(x, y, z) }
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

pub struct VertexCollection<F: Float, P: Point<F>> {
    vertices: Vec<Vertex<F, P>>,
    dimensions: Dimensions,
    write_order: WriteOrder,
}

impl<F: Float, P: Point<F>> Dimensioned for VertexCollection<F, P> {
    fn is_2d(&self) -> bool {
        self.dimensions.is_2d()
    }
}

impl<F: Float, P: Point<F>> VertexCollection<F, P> {
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }

    pub fn total_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self, v: Vertex<F, P>) {
        self.vertices.push(v);
    }
}

pub struct VertexCollectionBuilder {
    nx: usize, 
    ny: usize, 
    nz: Option<usize>,
    write_order: WriteOrder,
}

impl VertexCollectionBuilder {
    pub fn new(nx: usize, ny: usize) -> Self {
        VertexCollectionBuilder {
            nx, ny, nz: None, write_order: WriteOrder::XYZ,
        }
    }

    pub fn with_z(mut self, nz: usize) -> Self {
        self.nz = Some(nz);
        self
    }

    pub fn with_write_order(mut self, write_order: WriteOrder) -> Self {
        self.write_order = write_order;
        self
    }

    pub fn build_2d<F: Float>(self) -> VertexCollection<F, Point2D<F>> {
        VertexCollection {
            vertices: Vec::new(),
            dimensions: Dimensions::Two { nx: self.nx, ny: self.ny },
            write_order: self.write_order,
        }
    }

    pub fn build_3d<F: Float>(self) -> VertexCollection<F, Point3D<F>> {
        if self.nz.is_none() {
            panic!("can't create a 3d vertex collection without specifying nz")
        }
        VertexCollection {
            vertices: Vec::new(),
            dimensions: Dimensions::Three { nx: self.nx, ny: self.ny, nz: self.nz.unwrap() },
            write_order: self.write_order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_2d() {

    }
}