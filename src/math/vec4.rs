use std::ops;
use num::clamp;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4 {
            x,
            y,
            z,
            w,
        }
    }

    pub fn from_vec3(vec3: &Vec3, w: f64) -> Self {
        Vec4 {
            x: vec3.x.clone(),
            y: vec3.y.clone(),
            z: vec3.z.clone(),
            w,
        }
    }

    pub const fn one() -> Self {
        Vec4::new(1., 1., 1., 1.)
    }

    pub const fn zero() -> Self {
        Vec4::new(0., 0., 0., 0.)
    }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn sqr_length(&self) -> f64 {
        Vec4::dot(self, self)
    }

    pub fn length(&self) -> f64 {
        self.sqr_length().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    pub fn abs(&self) -> Self {
        Vec4 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
            w: self.w.abs(),
        }
    }

    pub fn min(a: &Self, b: f64) -> Self {
        Vec4 {
            x: a.x.min(b),
            y: a.y.min(b),
            z: a.z.min(b),
            w: a.w.min(b),
        }
    }

    pub fn max(a: &Self, b: f64) -> Self {
        Vec4 {
            x: a.x.max(b),
            y: a.y.max(b),
            z: a.z.max(b),
            w: a.w.max(b),
        }
    }

    pub fn max_element(&self) -> f64 {
        return self.x.max(self.y).max(self.z).max(self.w);
    }

    pub fn min_element(&self) -> f64 {
        return self.x.min(self.y).min(self.z).max(self.w);
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Vec4 {
            x: clamp(self.x, min, max),
            y: clamp(self.y, min, max),
            z: clamp(self.z, min, max),
            w: clamp(self.w, min, max),
        }
    }

    pub fn q_square(&self) -> Self {
        Vec4 {
            x: self.x * self.x - self.y * self.y - self.z * self.z - self.w * self.w,
            y: self.x * self.y * 2.,
            z: self.x * self.z * 2.,
            w: self.x * self.w * 2.,
        }
    }

    pub fn q_cube(&self) -> Self {
        let sqr_self = self * self;
        let new_x_factor = sqr_self.x - sqr_self.y * 3. - sqr_self.z * 3. - sqr_self.w * 3.;
        let new_other_factor = sqr_self.x * 3. - sqr_self.y - sqr_self.z - sqr_self.w;

        Vec4 {
            x: self.x * new_x_factor,
            y: self.y * new_other_factor,
            z: self.z * new_other_factor,
            w: self.w * new_other_factor,
        }
    }

    pub fn to_pixel_data(self) -> [u8; 3] {
        [(255.999 * self.x) as u8, (255.999 * self.y) as u8, (255.999 * self.z) as u8]
    }
}


fn internal_add_vec_vec(a: &Vec4, b: &Vec4) -> Vec4 {
    Vec4::new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.z)
}

fn internal_add_vec_scalar(a: &Vec4, b: f64) -> Vec4 {
    Vec4::new(a.x + b, a.y + b, a.z + b, a.w + b)
}

fn internal_sub_vec_vec(a: &Vec4, b: &Vec4) -> Vec4 {
    Vec4::new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.z)
}

fn internal_sub_vec_scalar(a: &Vec4, b: f64) -> Vec4 {
    Vec4::new(a.x - b, a.y - b, a.z - b, a.w - b)
}

fn internal_mul_vec_vec(a: &Vec4, b: &Vec4) -> Vec4 {
    Vec4::new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.z)
}

fn internal_mul_vec_scalar(a: &Vec4, b: f64) -> Vec4 {
    Vec4::new(a.x * b, a.y * b, a.z * b, a.w * b)
}

fn internal_div_vec_vec(a: &Vec4, b: &Vec4) -> Vec4 {
    Vec4::new(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.z)
}

fn internal_div_vec_scalar(a: &Vec4, b: f64) -> Vec4 {
    Vec4::new(a.x / b, a.y / b, a.z / b, a.w / b)
}

fn internal_neg_vec(a: &Vec4) -> Vec4 {
    Vec4::new(-a.x, -a.y, -a.z, -a.w)
}

// --- ADD ---

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<&Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: &Vec4) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: &Vec4) -> Self::Output {
        internal_add_vec_vec(&self, &rhs)
    }
}

impl ops::Add<f64> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: f64) -> Self::Output {
        internal_add_vec_scalar(&self, rhs)
    }
}

impl ops::Add<f64> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: f64) -> Self::Output {
        internal_add_vec_scalar(&self, rhs)
    }
}

// --- SUB ---

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<&Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: &Vec4) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: &Vec4) -> Self::Output {
        internal_sub_vec_vec(&self, &rhs)
    }
}

impl ops::Sub<f64> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: f64) -> Self::Output {
        internal_sub_vec_scalar(&self, rhs)
    }
}

impl ops::Sub<f64> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: f64) -> Self::Output {
        internal_sub_vec_scalar(&self, rhs)
    }
}


// --- MUL ---

impl ops::Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<Vec4> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<&Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}
impl ops::Mul<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: &Vec4) -> Self::Output {
        internal_mul_vec_vec(&self, &rhs)
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        internal_mul_vec_scalar(&self, rhs)
    }
}

impl ops::Mul<f64> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        internal_mul_vec_scalar(&self, rhs)
    }
}

// --- DIV ---

impl ops::Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<Vec4> for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<&Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: &Vec4) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<&Vec4> for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: &Vec4) -> Self::Output {
        internal_div_vec_vec(&self, &rhs)
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        internal_div_vec_scalar(&self, rhs)
    }
}

impl ops::Div<f64> for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        internal_div_vec_scalar(&self, rhs)
    }
}

// --- NEG ---

impl ops::Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        internal_neg_vec(&self)
    }
}

impl ops::Neg for &Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        internal_neg_vec(&self)
    }
}