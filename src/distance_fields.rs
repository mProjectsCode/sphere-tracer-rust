use num::complex::ComplexFloat;
use crate::vec3::Vec3;
use crate::vec4::Vec4;

#[derive(Debug, Clone)]
pub enum DistanceField {
    Sphere(Sphere),
    Cuboid(Cuboid),
    Torus(Torus),
    Plane(Plane),
    Julia(Julia),
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
            DistanceField::Julia(x) => x.get_distance(p),
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
        (p - self.pos).length() - self.size
    }
}

#[derive(Debug, Clone)]
pub struct Cuboid {
    pub pos: Vec3,
    pub size: Vec3,
}

impl DistanceFunction for Cuboid {
    fn get_distance(&self, p: &Vec3) -> f64 {
        let q = (p - self.pos).abs() - self.size;
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

// julia https://www.shadertoy.com/view/MsfGRr
// https://www.shadertoy.com/view/3tsyzl
/*
float map( in vec3 p, out vec4 oTrap, in vec4 c )
{
    vec4 z = vec4(p,0.0);
    float md2 = 1.0;
    float mz2 = dot(z,z);

    vec4 trap = vec4(abs(z.xyz),dot(z,z));

    float n = 1.0;
    for( int i=0; i<numIterations; i++ )
    {
        // dz -> 2·z·dz, meaning |dz| -> 2·|z|·|dz|
        // Now we take the 2.0 out of the loop and do it at the end with an exp2
        md2 *= 4.0*mz2;
        // z  -> z^2 + c
        z = qsqr(z) + c;

        trap = min( trap, vec4(abs(z.xyz),dot(z,z)) );

        mz2 = qlength2(z);
        if(mz2>4.0) break;
        n += 1.0;
    }

    oTrap = trap;

    return 0.25*sqrt(mz2/md2)*log(mz2);  // d = 0.5·|z|·log|z|/|z'|
}
 */

#[derive(Debug, Clone)]
pub struct Julia {
    pub pos: Vec3,
    pub iterations: i32,
    pub traps: bool,
    pub c: Vec4,
    pub cut: bool,
    pub cut_y: f64,
}

impl DistanceFunction for Julia {
    fn get_distance(&self, p: &Vec3) -> f64 {
        let p2 = &(p - self.pos);

        // if p2.length() > 2. {
        //     return p2.length() - 1.5;
        // }

        let mut z = Vec4::from_vec3(p2, 0.);
        let mut sqrt_derive_z = 1.;
        let mut m2 = 0.;
        let mut n = 0.;
        let mut o = 1e10;

        for i in 0..self.iterations {
            // z' = 3z² -> |z'|² = 9|z²|²
            sqrt_derive_z *= 9. * z.q_square().sqr_length();
            // z = z³ + c
            z = z.q_cube() + self.c;

            m2 = z.sqr_length();

            // orbit trapping : https://iquilezles.org/articles/orbittraps3d
            if self.traps {
                let new_o = ((z.x - 0.45).powi(2) + (z.z - 0.55).powi(2)).sqrt() - 0.1;

                o = f64::min(o, new_o);
            }

            if m2 > 256.0 {
                break;
            }

            n += 1.0;
        }

        let mut d = 0.25 * m2.ln() * (m2 / sqrt_derive_z).sqrt();

        if self.traps {
            d = f64::min(d, o);
        }

        if self.cut {
            d = f64::max(d, p.y);
        }

        d
    }
}