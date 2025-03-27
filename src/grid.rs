#[derive(Debug, Clone, Copy)]
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
    nx: usize, 
    ny: usize,
    points: Vec<GridPoint2D>, // vec containing all grid points
}

impl Grid2D {
    pub fn new(nx: usize, ny: usize) -> Self {
        Grid2D { nx, ny, points: Vec::with_capacity(nx * ny) }
    }

    pub fn add_point(&mut self, x: f64, y: f64) {
        let (i, j) = match self.get_next_ij() {
            Some(tuple) => tuple,
            None => return // need better handling of none return
        };

        let point = GridPoint2D::new(i, j, x, y);
        self.points.push(point);
    }

    pub fn num_pts(&self) -> usize {
        self.points.len()
    }

    pub fn extents(&self) -> (f64, f64, f64, f64) {
        let mut min_x: f64 = 0.0;
        let mut max_x: f64 = 0.0;
        let mut min_y: f64 = 0.0;
        let mut max_y: f64 = 0.0;

        for point in self.points.iter() {
            if point.x < min_x {
                min_x = point.x;
            } else if point.x > max_x {
                max_x = point.x;
            }

            if point.y < min_y {
                min_y = point.y;
            } else if point.y > max_y {
                max_y = point.y;
            }
        }

        (min_x, max_x, min_y, max_y)
    }

    fn get_next_ij(&self) -> Option<(usize, usize)> {
        // find last point in point vec, return (0, 0) if vec is empty
        let last = match self.points.last() {
            Some(point) => point,
            None => return Some((0, 0))
        };

        // check if we are at the maximum (i, j) as defined by nx and ny, return none if we are
        if last.i == self.nx - 1 && last.j == self.ny - 1 {
            return None
        } 

        // find the proceeding i and j values depending on what point was the last in the vec
        let next_i: usize;
        let next_j: usize;

        if last.i == self.nx - 1 {
            next_i = 0;
            next_j = last.j + 1;
        } else {
            next_i = last.i + 1;
            next_j = last.j;
        }

        Some((next_i, next_j))
    }
}