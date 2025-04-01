#![allow(dead_code)]

use std::ops::{Div, Sub};
use num::Num;

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

type RowVector<S: Scalar, const COLS: usize> = Matrix<S, 1, COLS>;
type ColumnVector<S: Scalar, const ROWS: usize> = Matrix<S, ROWS, 1>;
type SquareMatrix<S: Scalar, const DIMS: usize> = Matrix<S, DIMS, DIMS>;
type Vector<S: Scalar, const LENGTH: usize> = ColumnVector<S, LENGTH>;

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