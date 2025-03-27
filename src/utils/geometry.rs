#![allow(dead_code)]

use num_complex::Complex;

pub struct Cartesian2D {
    x: f64,
    y: f64,
}

pub trait Line2D {
    fn eqn(&self) -> impl Fn(f64) -> f64;
    fn solve(&self, x: f64) -> f64;
}

// straight line stuff //
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

    fn solve(&self, x: f64) -> f64 {
        (self.eqn())(x)
    }
} 

impl std::fmt::Display for StraightLine2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.m != 0.0 && self.c != 0.0 {
            write!(f, "y = {:.2}x + {:.2}", self.m, self.c)
        } else if self.m != 0.0 && self.c == 0.0 {
            write!(f, "y = {:.2}x", self.m)
        } else if self.m == 0.0 && self.c != 0.0 {
            write!(f, "y = {:.2}", self.c)
        } else {
            write!(f, "y = 0")
        }
    }
}

impl std::fmt::Debug for StraightLine2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
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

impl Polynomial {
    pub fn roots(&self) -> Vec<Complex<f64>> {
        // if order is 0, there's no root to compute
        let n = self.order();
        if n == 0 {
            return vec![];
        }

        // convert polynomial to monic form
        // coefficients are assumed in descending order: a0 x^n + a1 x^(n-1) + ... + a{n}
        let leading = self.coefs[0];
        // build monic coefficients as complex numbers
        let b: Vec<Complex<f64>> = self.coefs
            .iter()
            .map(|&c| Complex::new(c / leading, 0.0))
            .collect();

        // initialize roots with equally spaced points on a circle.
        // using a small radius may help with convergence.
        let n_usize = n;
        let mut roots: Vec<Complex<f64>> = (0..n_usize)
            .map(|k| {
                let theta = 2.0 * std::f64::consts::PI * k as f64 / n_usize as f64;
                // initial guess; adjust the radius as needed
                Complex::from_polar(&1.0, &theta)
            })
            .collect();

        let max_iters = 1000;
        let tol = 1e-8;

        for _ in 0..max_iters {
            let mut converged = true;
            let mut new_roots = roots.clone();
            for i in 0..n_usize {
                let r_i = roots[i];

                // evaluate the monic polynomial at r_i:
                // p(r) = r^n + b[1]*r^(n-1) + ... + b[n]
                let mut p_val = Complex::new(1.0, 0.0);
                for coef in b.iter().skip(1) {
                    p_val = p_val * r_i + coef;
                }

                // compute the product prod_{j != i} (r_i - r_j)
                let prod = roots.iter().enumerate()
                    .filter(|(j, _)| *j != i)
                    .fold(Complex::new(1.0, 0.0), |acc, (_, &r_j)| acc * (r_i - r_j));

                // avoid division by zero, update the root
                let delta = p_val / prod;
                new_roots[i] = r_i - delta;
                if delta.norm() > tol {
                    converged = false;
                }
            }
            roots = new_roots;
            if converged {
                break;
            }
        }

        roots
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

    fn solve(&self, x: f64) -> f64 {
        (self.eqn())(x)
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

            if first_term {
                write!(f, "y = ")?;
                first_term = false;
                if coeff < 0.0 {
                    write!(f, "-")?;
                }
            } else {
                if coeff > 0.0 {
                    write!(f, " + ")?;
                } else {
                    write!(f, " - ")?;
                }
            }
            
            let power: usize = order - i;

            if power == 0 {
                write!(f, "{:.2}", coeff.abs())?;
            } else if power == 1 {
                write!(f, "{:.2}x", coeff.abs())?;
            } else {
                write!(f, "{:.2}x^{}", coeff.abs(), power)?;
            }
        }

        if first_term {
            write!(f, "y = 0")?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// b-spline stuff // 
pub struct BSpline {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_straight_line() {
        let line1 = StraightLine2D::new(2.0, 3.0);
        let line2 = StraightLine2D::new(1.0, 0.0);
        println!("{}", line1);
        println!("{}", line2);
    }

    #[test]
    fn print_poly() {
        let poly = Polynomial::new(vec![1.0, 2.0, -3.0, 4.0]);
        println!("{}", poly);
    }
}