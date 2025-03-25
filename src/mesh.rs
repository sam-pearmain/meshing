#[derive(Debug)]
pub struct GridPoint2D {
    i: usize, 
    j: usize, 
    x: f64,
    y: f64,
}

impl GridPoint2D {
    pub fn new(i: usize, j: usize, x: f64, y: f64) -> Self {
        GridPoint2D { i, j, x, y }
    }
}

#[derive(Debug)]
pub struct Grid2D {
    grid_points: Vec<GridPoint2D>,
}

impl Grid2D {

}