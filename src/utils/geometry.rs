#![allow(dead_code)]

pub struct Cartesian2D {
    x: f64,
    y: f64,
}

pub trait Line2D {
    fn eqn(&self) -> impl Fn(f64) -> f64;
}

pub struct StraightLine2D {
    m: f64, // gradient
    c: f64, // y-intercept
}

impl StraightLine2D {
    pub fn new(m: f64, c: f64) -> Self {
        StraightLine2D { m, c }
    }

    pub fn from_cartesian_points(p1: Cartesian2D, p2: Cartesian2D) -> Self {
        let m: f64 = (p2.y - p1.y) / (p2.x - p2.x);
        let c: f64 = p1.y - m * p1.x;
        StraightLine2D { m, c }
    }
}

impl Line2D for StraightLine2D {
    fn eqn(&self) -> impl Fn(f64) -> f64 {
        move |x: f64| self.m * x + self.c
    }
} 

// polynomial stuff // 
pub struct Polynomial {
    coefs: Vec<f64>,
}

impl Polynomial {
    pub fn new(coefs: Vec<f64>) -> Self {
        Polynomial { coefs }
    }

    pub fn order(&self) -> usize {
        self.coefs.len() - 1
    }
}

impl Line2D for Polynomial {
    fn eqn(&self) -> impl Fn(f64) -> f64 {
        let coefs: Vec<f64> = self.coefs.clone();
        move |x: f64| {
            let mut y: f64 = 0.0;
            for coeff in coefs.iter() {
                y = y * x + coeff;
            }
            y
        }
    }
}

impl std::fmt::Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let order: usize = self.order();
        let mut first_term: bool = true;
        for (i, &coeff) in self.coefs.iter().enumerate() {
            if coeff == 0.0 {
                continue;
            }
            if !first_term {
                write!(f, " + ")?;
            }
            first_term = false;
            let power: usize = order - i;
            if power == 0 {
                write!(f, "{:.2}", coeff)?;
            } else if power == 1 {
                write!(f, "{:.2}x", coeff)?;
            } else {
                write!(f, "{:.2}x^{}", coeff, power)?;
            }
        }

        if first_term {
            write!(f, "0")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}