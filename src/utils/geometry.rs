#![allow(dead_code)]

use num_complex::Complex;
use crate::utils::plotting::plot_injective_function;

pub struct Cartesian2D {
    x: f64,
    y: f64,
}

pub trait Line2D {
    fn eqn(&self) -> impl Fn(f64) -> f64;
    fn solve(&self, x: f64) -> f64;
    fn plot(&self, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.plot_bounded((-10.0, 10.0), (-10.0, 10.0), file_name)
    }
    fn plot_bounded(&self, x_range: (f64, f64), y_range: (f64, f64), file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        plot_injective_function(self.eqn(), x_range, y_range, file_name)
    }
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

    pub fn roots(&self) -> Vec<Complex<f64>> {
        // if order is 0, there's no root to compute
        let n: usize = self.order();
        if n == 0 {
            return vec![];
        }

        // convert polynomial to monic form
        // coefficients are assumed in descending order: a0 x^n + a1 x^(n-1) + ... + a{n}
        let leading: f64 = self.coefs[0];
        // build monic coefficients as complex numbers
        let b: Vec<Complex<f64>> = self.coefs
            .iter()
            .map(|&c| Complex::new(c / leading, 0.0))
            .collect();

        // initialize roots with equally spaced points on a circle.
        // using a small radius may help with convergence.
        let n_usize: usize = n;
        let mut roots: Vec<Complex<f64>> = (0..n_usize)
            .map(|k: usize| {
                let theta: f64 = 2.0 * std::f64::consts::PI * k as f64 / n_usize as f64;
                // initial guess; adjust the radius as needed
                Complex::from_polar(1.0, theta)
            })
            .collect();

        let max_iters: i32 = 1000;
        let tol: f64 = 1e-12;

        for _ in 0..max_iters {
            let mut converged: bool = true;
            let mut new_roots: Vec<Complex<f64>> = roots.clone();
            for i in 0..n_usize {
                let r_i: Complex<f64> = roots[i];

                // evaluate the monic polynomial at r_i:
                // p(r) = r^n + b[1]*r^(n-1) + ... + b[n]
                let mut p_val: Complex<f64> = Complex::new(1.0, 0.0);
                for coef in b.iter().skip(1) {
                    p_val = p_val * r_i + coef;
                }

                // compute the product prod_{j != i} (r_i - r_j)
                let prod: Complex<f64> = roots.iter().enumerate()
                    .filter(|(j, _)| *j != i)
                    .fold(Complex::new(1.0, 0.0), |acc, (_, &r_j)| acc * (r_i - r_j));

                // avoid division by zero, update the root
                let delta: Complex<f64> = p_val / prod;
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

    pub fn real_roots(&self) -> Vec<f64> {
        // tolerance for considering a complex root to be real
        let tol: f64 = 1e-8;
        self.roots()
            .into_iter()
            .filter(|r| r.im.abs() < tol)
            .map(|r| r.re)
            .collect()
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
    use num_complex::Complex;

    const TOL: f64 = 1e-6;

    fn poly_near_zero(poly: &Polynomial, root: Complex<f64>) -> bool {
        let f_val = (poly.eqn())(root.re);
        f_val.abs() < TOL
    }

    #[test]
    fn test_quadratic_roots() {
        // polynomial: x^2 - 5x + 6 = (x - 2)(x - 3)
        let poly = Polynomial::new(vec![1.0, -5.0, 6.0]);
        let roots = poly.roots();
        assert_eq!(roots.len(), 2);
        let expected_roots = vec![
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        for exp in expected_roots {
            let found = roots.iter().any(|r| (r - exp).norm() < TOL);
            assert!(found, "expected root {} not found", exp);
        }
    }

    #[test]
    fn test_cubic_roots() {
        // polynomial: x^3 - 6x^2 + 11x - 6 = (x - 1)(x - 2)(x - 3)
        let poly = Polynomial::new(vec![1.0, -6.0, 11.0, -6.0]);
        let roots = poly.roots();
        assert_eq!(roots.len(), 3);
        let expected_roots = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
        ];
        for exp in expected_roots {
            let found = roots.iter().any(|r| (r - exp).norm() < TOL);
            assert!(found, "expected root {} not found", exp);
        }
    }

    #[test]
    fn test_fourth_order_roots() {
        // polynomial: x^4 - 10x^3 + 35x^2 - 50x + 24 = (x - 1)(x - 2)(x - 3)(x - 4)
        let poly = Polynomial::new(vec![1.0, -10.0, 35.0, -50.0, 24.0]);
        let roots = poly.roots();
        assert_eq!(roots.len(), 4);
        let expected_roots = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];
        for exp in expected_roots {
            let found = roots.iter().any(|r| (r - exp).norm() < TOL);
            assert!(found, "expected root {} not found", exp);
        }
        println!("{:?}", poly.real_roots());
    }
    
    #[test]
    fn test_tenth_order_real_roots() {
        // polynomial: x^10 - 2.30x^9 + 3.45x^8 - 4.56x^7 + 1.23x^6 - 0.78x^5 + 0.56x^4 - 0.34x^3 + 0.12x^2 - 0.05x + 1.00
        let poly = Polynomial::new(vec![
            1.0,    // coefficient for x^10
            -2.3,   // coefficient for x^9
            3.45,   // coefficient for x^8
            -4.56,  // coefficient for x^7
            1.23,   // coefficient for x^6
            -0.78,  // coefficient for x^5
            0.56,   // coefficient for x^4
            -0.34,  // coefficient for x^3
            0.12,   // coefficient for x^2
            -0.05,  // coefficient for x^1
            1.0,    // constant term (x^0)
        ]);
        let real_roots = poly.real_roots();
        println!("{:?}", real_roots);
    }

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