use crate::distance_fields::{DistanceField, DistanceFunction};
use crate::ray::Ray;
use crate::vector::Vec3;
use crate::vector::Vec4;

pub struct RayMarcher {
    // quality
    pub max_iterations: i32,
    pub max_distance: f64,
    pub accuracy: f64,

    // misc
    pub debug: bool,

    // normals
    pub normal_accuracy: f64,
    pub offset_x: Vec3,
    pub offset_y: Vec3,
    pub offset_z: Vec3,

    // scene
    pub scene: DistanceField,
    pub obj_color: Vec3,

    // sun light
    pub light_dir: Vec3,
    pub light_color: Vec3,
    pub light_intensity: f64,

    // indirect light
    pub bg_light_color: Vec3,
    pub bg_light_intensity: f64,
}

pub fn create_ray_marcher(scene: DistanceField) -> RayMarcher {
    let max_iterations = 200;
    let max_distance = 64.;
    let accuracy = 0.001;

    let debug = false;

    let obj_color = Vec3::new(1., 0., 1.);

    let normal_accuracy = 0.00001;
    let offset_x = Vec3::new(normal_accuracy, 0., 0.);
    let offset_y = Vec3::new(0., normal_accuracy, 0.);
    let offset_z = Vec3::new(0., 0., normal_accuracy);

    let light_dir = Vec3::new(1., -0.1, -0.2).normalize();
    let light_color = Vec3::new(0.7, 0.7, 1.);
    let light_intensity = 1.;

    let bg_light_color = Vec3::new(1., 1., 1.);
    let bg_light_intensity = 0.2;


    RayMarcher {
        max_iterations,
        max_distance,
        accuracy,

        debug,

        obj_color,

        normal_accuracy,
        offset_x,
        offset_y,
        offset_z,

        scene,

        light_dir,
        light_color,
        light_intensity,

        bg_light_color,
        bg_light_intensity,

    }
}

impl RayMarcher {
    pub fn ray_marching(&self, ray: Ray) -> Vec4 {
        let mut result: Vec4 = Vec4::one();

        let mut t: f64 = 0.;

        for i in 0..self.max_iterations {
            if t > self.max_distance {
                if self.debug {
                    result = Vec4::one() * i as f64 / self.max_iterations as f64;
                    break;
                } else {
                    result = Vec4::zero();
                    break;
                }
            }

            let p = ray.orig + ray.dir * t;
            let d = self.distance_field(&p);

            if d < self.accuracy {
                if self.debug {
                    result = Vec4::one() * i as f64 / self.max_iterations as f64;
                    break;
                } else {
                    result = self.shading(&p);
                    break;
                }
            }

            t += d;
        }

        result
    }

    fn distance_field(&self, p: &Vec3) -> f64 {
        self.scene.get_distance(p)
    }

    fn get_normal(&self, p: &Vec3) -> Vec3 {
        Vec3::new(
            self.distance_field(&(p + &self.offset_x)) - self.distance_field(&(p - &self.offset_x)),
            self.distance_field(&(p + &self.offset_y)) - self.distance_field(&(p - &self.offset_y)),
            self.distance_field(&(p + &self.offset_z)) - self.distance_field(&(p - &self.offset_z)),
        ).normalize()
    }

    fn shading(&self, p: &Vec3) -> Vec4 {
        let n = self.get_normal(&p);
        let shadow = self.shadow(&p, &n);

        // Vec4::from_vec3(&n, 1.)

        let sun_light = self.obj_color * (self.light_color * Vec3::dot(&(-&self.light_dir), &n).clamp(0., 1.) * self.light_intensity * shadow);
        let bg_light = self.obj_color * (self.bg_light_color * self.bg_light_intensity);

        Vec4::from_vec3(&(sun_light + bg_light), 1.)
    }

    fn shadow(&self, p: &Vec3, n: &Vec3) -> f64 {
        let sro = p + &(n * self.accuracy);
        let sr = Ray::new(sro, -self.light_dir);

        let mut t: f64 = 0.;

        for i in 0..self.max_iterations {
            if t > self.max_distance {
                return 1.;
            }

            let p = sr.orig + sr.dir * t;
            let d = self.distance_field(&p);

            if d < self.accuracy {
                return 0.;
            }

            t += d;
        }

        return 1.;
    }
}

