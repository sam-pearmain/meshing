#![allow(dead_code)]

use std::ops::{Div, Sub};
use num::{Num, Signed};

pub trait Scalar: Num + Clone + Copy + Default + std::fmt::Debug {}

impl Scalar for f32   {}
impl Scalar for f64   {}
impl Scalar for i8    {}
impl Scalar for i16   {}
impl Scalar for i32   {}
impl Scalar for i64   {}
impl Scalar for i128  {}
impl Scalar for u8    {}
impl Scalar for u16   {}
impl Scalar for u32   {}
impl Scalar for u64   {}
impl Scalar for u128  {}
impl Scalar for usize {}

#[derive(Clone)]
pub struct Matrix<S: Scalar, const ROWS: usize, const COLS: usize> {
    data: Vec<S>,
}

type Vector<S: Scalar, const LENGTH: usize> = ColumnVector<S, LENGTH>;
type RowVector<S: Scalar, const COLS: usize> = Matrix<S, 1, COLS>;
type ColumnVector<S: Scalar, const ROWS: usize> = Matrix<S, ROWS, 1>;
type SquareMatrix<S: Scalar, const DIMS: usize> = Matrix<S, DIMS, DIMS>;

impl<S: Scalar, const ROWS: usize, const COLS: usize> std::fmt::Debug for Matrix<S, ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix::{} [{}x{}]", std::any::type_name::<S>(), ROWS, COLS)?;
        Ok(())
    }
}

impl<S: Scalar, const ROWS: usize, const COLS: usize> std::fmt::Display for Matrix<S, ROWS, COLS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..ROWS {
            for j in 0..COLS {
                let index = i * COLS + j;
                write!(f, " {:?} ", self.data[index])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
} 

impl<S: Scalar, const ROWS: usize, const COLS: usize> Matrix<S, ROWS, COLS> {
    pub fn new() -> Self {
        Matrix { data: vec![S::default(); ROWS * COLS], }
    }

    pub fn zeros() -> Self {
        Matrix { data: vec![S::zero(); ROWS * COLS], }
    }

    pub fn ones() -> Self {
        Matrix { data: vec![S::one(); ROWS * COLS], }
    }

    pub fn fill(value: S) -> Self {
        Matrix { data: vec![value; ROWS * COLS], }
    }

    pub fn from_vec(data: &Vec<S>) -> Result<Self, &'static str> {
        if data.len() != ROWS * COLS {
            return Err("vec length does not match matrix dimensions");
        }
        Ok(Matrix { data: data.to_vec() })
    }

    pub fn get(&self, row: usize, col: usize) -> Result<S, &'static str> {
        if row < ROWS && col < COLS {
            Ok(self.data[row * COLS + col])
        } else {
            Err("index out of bounds")
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: S) -> Result<(), &'static str> {
        if row < ROWS && col < COLS {
            self.data[row * COLS + col] = value;
            Ok(())
        } else {
            Err("index out of bounds")
        }
    }

    pub fn transpose(&self) -> Result<Matrix<S, COLS, ROWS>, &'static str> {
        let mut transpose = Matrix::<S, COLS, ROWS>::zeros();
        for i in 0..ROWS {
            for j in 0..COLS {
                transpose.set(j, i, self.get(i, j)?)?;
            }
        }
        Ok(transpose)
    }

    pub fn rows(&self) -> usize {
        ROWS
    }

    pub fn cols(&self) -> usize {
        COLS
    }

    pub fn dims(&self) -> (usize, usize) {
        (ROWS, COLS)
    }
}

impl<S: Scalar, const DIMS: usize> SquareMatrix<S, DIMS> {
    fn identity() -> Self {
        let mut data = vec![S::zero(); DIMS * DIMS];
        for i in 0..DIMS {
            data[i * DIMS + i] = S::one();
        }
        Matrix { data }
    }
} 

impl<S: Scalar + num::Signed, const DIMS: usize> SquareMatrix<S, DIMS> {
    fn inverse(&self) -> Result<Self, &'static str> {
        todo!()
    }
    
    fn determinant(&self) -> S {
        todo!()
    }
}

pub struct AugmentedMatrix<S: Scalar, const DIMS: usize> {
    a: SquareMatrix<S, DIMS>,
    b: Vector<S, DIMS>,
}

impl<S: Scalar + Div<Output = S> + Sub<Output = S>, const DIMS: usize> AugmentedMatrix<S, DIMS> {
    fn assemble(a: &SquareMatrix<S, DIMS>, b: &Vector<S, DIMS>) -> Self {
        AugmentedMatrix { 
            a: a.clone(), 
            b: b.clone(), 
        }
    }
    
    fn lu_decomposition(&self) -> (SquareMatrix<S, DIMS>, SquareMatrix<S, DIMS>) {
        let mut l = SquareMatrix::<S, DIMS>::identity();
        let mut u = self.a.clone();

        for i in 0..DIMS {
            for j in i + 1..DIMS {
                let factor = u.data[j * DIMS + i] / u.data[i * DIMS + i];
                l.data[j * DIMS + i] = factor;

                for k in i..DIMS {
                    u.data[j * DIMS + k] = u.data[j * DIMS + k] - factor * u.data[i * DIMS + k];
                }
            }
        }

        (l, u)
    }
}

impl<S: Scalar, const ROWS: usize, const COLS: usize> std::ops::Add for Matrix<S, ROWS, COLS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data = self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Matrix { data }
    }
}

impl<S: Scalar + Signed, const ROWS: usize, const COLS: usize> std::ops::Sub for Matrix<S, ROWS, COLS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data = self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| *a - *b)
            .collect();
        Matrix { data }
    }
}

impl<S: Scalar, const ROWS: usize, const COMMON: usize, const COLS: usize> std::ops::Mul<Matrix<S, COMMON, COLS>> for Matrix<S, ROWS, COMMON> {
    type Output = Matrix<S, ROWS, COLS>;

    fn mul(self, rhs: Matrix<S, COMMON, COLS>) -> Self::Output {
        let mut result = vec![S::zero(); ROWS * COLS];
        for i in 0..ROWS {
            for j in 0..COLS {
                for k in 0..COMMON {
                    result[i * COLS + j] = result[i * COLS + j]
                        + self.data[i * COMMON + k] * rhs.data[k * COLS + j];
                }
            }
        }
        Matrix { data: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let m = Matrix::<f64, 4, 4>::new();
        println!("{}", m);

        let m = Vector::<f32, 10>::ones();
        println!("{:?}", m);
    }
}