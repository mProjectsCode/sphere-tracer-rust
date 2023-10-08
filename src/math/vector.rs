use std::ops;

use num::clamp;
use num::integer::Roots;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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

    pub fn dot(a: Self, b: Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn sqr_length(self) -> f64 {
        Vec4::dot(self, self)
    }

    pub fn length(self) -> f64 {
        self.sqr_length().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn clamp(self, min: Self, max: Self) -> Self {
        Vec4 {
            x: clamp(self.x, min.x, max.x),
            y: clamp(self.y, min.y, max.y),
            z: clamp(self.z, min.z, max.z),
            w: clamp(self.w, min.w, max.w),
        }
    }

    pub fn to_pixel_data(self) -> [u8; 3] {
        [(255.999 * self.x) as u8, (255.999 * self.y) as u8, (255.999 * self.z) as u8]
    }
}

// --- Vec3 Traits ---

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// --- Vec3 Ref Traits ---

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// --- Vec4 Traits ---

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f64) -> Self::Output {
        Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w * rhs)
    }
}