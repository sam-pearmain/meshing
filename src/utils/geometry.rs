#![allow(dead_code)]

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