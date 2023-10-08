use crate::vector::Vec3;

#[derive(Debug, Clone)]
pub enum DistanceField {
    Sphere(Sphere),
    Cuboid(Cuboid),
    Torus(Torus),
    Plane(Plane),
    Union(Box<Union>),
    Subtraction(Box<Subtraction>),
    Intersection(Box<Intersection>),
}

pub trait DistanceFunction {
    fn get_distance(&self, p: &Vec3) -> f64;
}

impl DistanceFunction for DistanceField {
    fn get_distance(&self, p: &Vec3) -> f64 {
        match self {
            DistanceField::Sphere(x) => x.get_distance(p),
            DistanceField::Cuboid(x) => x.get_distance(p),
            DistanceField::Torus(x) => x.get_distance(p),
            DistanceField::Plane(x) => x.get_distance(p),
            DistanceField::Union(x) => x.get_distance(p),
            DistanceField::Subtraction(x) => x.get_distance(p),
            DistanceField::Intersection(x) => x.get_distance(p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub pos: Vec3,
    pub size: f64,
}

impl DistanceFunction for Sphere {
    fn get_distance(&self, p: &Vec3) -> f64 {
        (p - &self.pos).length() - self.size
    }
}

#[derive(Debug, Clone)]
pub struct Cuboid {
    pub pos: Vec3,
    pub size: Vec3,
}

impl DistanceFunction for Cuboid {
    fn get_distance(&self, p: &Vec3) -> f64 {
        let q = (p - &self.pos).abs() - self.size;
        (Vec3::max(&q, 0.) + q.max_element().min(0.)).length()
    }
}

#[derive(Debug, Clone)]
pub struct Torus {
    pub pos: Vec3,
    pub outer_size: f64,
    pub inner_size: f64,
}

impl DistanceFunction for Torus {
    fn get_distance(&self, p: &Vec3) -> f64 {
        let q = Vec3::new((p.x * p.x + p.z * p.z).sqrt() - self.outer_size, p.y, 0.);
        q.length() - self.inner_size
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub h: f64,
}

impl DistanceFunction for Plane {
    fn get_distance(&self, p: &Vec3) -> f64 {
        Vec3::dot(p, &self.normal) + self.h
    }
}

#[derive(Debug, Clone)]
pub struct Union {
    pub a: DistanceField,
    pub b: DistanceField,
}

impl DistanceFunction for Union {
    fn get_distance(&self, p: &Vec3) -> f64 {
        f64::min(self.a.get_distance(p), self.b.get_distance(p))
    }
}

#[derive(Debug, Clone)]
pub struct Subtraction {
    pub a: DistanceField,
    pub b: DistanceField,
}

impl DistanceFunction for Subtraction {
    fn get_distance(&self, p: &Vec3) -> f64 {
        f64::max(-self.a.get_distance(p), self.b.get_distance(p))
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    pub a: DistanceField,
    pub b: DistanceField,
}

impl DistanceFunction for Intersection {
    fn get_distance(&self, p: &Vec3) -> f64 {
        f64::max(self.a.get_distance(p), self.b.get_distance(p))
    }
}