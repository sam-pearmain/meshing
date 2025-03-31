#![allow(dead_code)]

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize, 
}

impl Matrix {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Matrix { 
            data: vec![0.0; nrows * ncols],
            rows: nrows,
            cols: ncols, 
        }
    }

    pub fn fill(&mut self, value: f64) {
        for element in self.data.iter_mut() {
            *element = value;
        }
    }

    pub fn element(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    
}

impl std::ops::Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("matrices must have same dimensions for addition");
        }

        let data: Vec<f64> = self.data.clone()
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix {
            data, 
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.rows != rhs.rows || self.cols != rhs.cols {
            panic!("matrices must have same dimensions for subtraction");
        }

        let data: Vec<f64> = self.data.clone()
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Matrix {
            data, 
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("number of columns in the first matrix must equal the number of rows in the second matrix for multiplication");
        }

        let mut result = Matrix::new(self.rows, rhs.cols);

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.element(i, k) * rhs.element(k, j);
                }
                result.data[i * rhs.cols + j] = sum;
            }
        }

        result
    }
}