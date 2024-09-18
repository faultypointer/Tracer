use core::f64;
use std::fmt;
use std::ops::{self, MulAssign};

use crate::rtweeknd;

#[derive(Clone, Copy)]
pub struct Vector {
    e: [f64; 3],
}

pub type Point = Vector;

impl Vector {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vector { e: [e0, e1, e2] }
    }

    pub fn zero() -> Self {
        Vector { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.e[0]) < s && f64::abs(self.e[1]) < s && f64::abs(self.e[2]) < s
    }
}

impl Vector {
    pub fn random() -> Self {
        Vector {
            e: [rtweeknd::random(), rtweeknd::random(), rtweeknd::random()],
        }
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Vector {
            e: [
                rtweeknd::random_in_range(min, max),
                rtweeknd::random_in_range(min, max),
                rtweeknd::random_in_range(min, max),
            ],
        }
    }

    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            let len_p = p.length_squared();
            if len_p > 1e-160 && len_p <= 1.0 {
                return p / f64::sqrt(len_p);
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vector) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        if Vector::dot(on_unit_sphere, *normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return -on_unit_sphere;
        }
    }
}

impl ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}
impl ops::Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}
impl ops::Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}
impl ops::Mul for Vector {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}
impl ops::Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl ops::Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}
impl ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.mul_assign(1.0 / rhs);
    }
}

impl Vector {
    pub fn dot(lhs: Self, rhs: Self) -> f64 {
        lhs.e[0] * rhs.e[0] + lhs.e[1] * rhs.e[1] + lhs.e[2] * rhs.e[2]
    }

    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Vector {
            e: [
                lhs.e[1] * rhs.e[2] - lhs.e[2] * rhs.e[1],
                lhs.e[2] * rhs.e[0] - lhs.e[0] * rhs.e[2],
                lhs.e[0] * rhs.e[1] - lhs.e[1] * rhs.e[0],
            ],
        }
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v - 2.0 * Self::dot(v, n) * n
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
