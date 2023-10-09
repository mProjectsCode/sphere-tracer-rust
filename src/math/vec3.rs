use std::ops;

use num::clamp;
use num::integer::Roots;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub const fn one() -> Self {
        Vec3::new(1., 1., 1.)
    }

    pub const fn zero() -> Self {
        Vec3::new(0., 0., 0.)
    }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Self, b: &Self) -> Vec3 {
        Vec3 {
            x: a.y*b.z - a.z*b.y,
            y: a.z*b.x - a.x*b.z,
            z: a.x*b.y - a.y*b.x,
        }
    }

    pub fn sqr_length(&self) -> f64 {
        Vec3::dot(self, self)
    }

    pub fn length(&self) -> f64 {
        self.sqr_length().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    pub fn abs(&self) -> Self {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn min(a: &Self, b: f64) -> Self {
        Vec3 {
            x: a.x.min(b),
            y: a.y.min(b),
            z: a.z.min(b),
        }
    }

    pub fn max(a: &Self, b: f64) -> Self {
        Vec3 {
            x: a.x.max(b),
            y: a.y.max(b),
            z: a.z.max(b),
        }
    }

    pub fn max_element(&self) -> f64 {
        return self.x.max(self.y).max(self.z);
    }

    pub fn min_element(&self) -> f64 {
        return self.x.min(self.y).min(self.z);
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Vec3 {
            x: clamp(self.x, min, max),
            y: clamp(self.y, min, max),
            z: clamp(self.z, min, max),
        }
    }
}

fn internal_add_vec_vec(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)
}

fn internal_add_vec_scalar(a: &Vec3, b: f64) -> Vec3 {
    Vec3::new(a.x + b, a.y + b, a.z + b)
}

fn internal_sub_vec_vec(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)
}

fn internal_sub_vec_scalar(a: &Vec3, b: f64) -> Vec3 {
    Vec3::new(a.x - b, a.y - b, a.z - b)
}

fn internal_mul_vec_vec(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

fn internal_mul_vec_scalar(a: &Vec3, b: f64) -> Vec3 {
    Vec3::new(a.x * b, a.y * b, a.z * b)
}

fn internal_div_vec_vec(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z)
}

fn internal_div_vec_scalar(a: &Vec3, b: f64) -> Vec3 {
    Vec3::new(a.x / b, a.y / b, a.z / b)
}

fn internal_neg_vec(a: &Vec3) -> Vec3 {
    Vec3::new(-a.x, -a.y, -a.z)
}

// --- ADD ---

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        internal_add_vec_scalar(&self, rhs)
    }
}

impl ops::Add<f64> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        internal_add_vec_scalar(&self, rhs)
    }
}

// --- SUB ---

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        internal_sub_vec_scalar(&self, rhs)
    }
}

impl ops::Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        internal_sub_vec_scalar(&self, rhs)
    }
}


// --- MUL ---

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<&Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        internal_mul_vec_scalar(&self, rhs)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        internal_mul_vec_scalar(&self, rhs)
    }
}

// --- DIV ---

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
       internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<&Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        internal_div_vec_scalar(&self, rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        internal_div_vec_scalar(&self, rhs)
    }
}

// --- NEG ---

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        internal_neg_vec(&self)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        internal_neg_vec(&self)
    }
}