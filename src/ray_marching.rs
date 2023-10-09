use crate::distance_fields::{DistanceField, DistanceFunction};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec4::Vec4;

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

    // shadow
    pub shadow_dist_min: f64,
    pub shadow_dist_max: f64,
    pub shadow_fuzziness: f64,

    // AO
    pub ao_step_size: f64,
    pub ao_intensity: f64,
    pub ao_iterations: i32,
}

pub fn create_ray_marcher(scene: DistanceField) -> RayMarcher {
    let max_iterations = 4000;
    let max_distance = 7.;
    let accuracy = 0.00001;

    let debug = false;

    let obj_color = Vec3::new(1., 1., 1.);

    let normal_accuracy = 0.000001;
    let offset_x = Vec3::new(normal_accuracy, 0., 0.);
    let offset_y = Vec3::new(0., normal_accuracy, 0.);
    let offset_z = Vec3::new(0., 0., normal_accuracy);

    let light_dir = Vec3::new(0.5, -1., 0.5).normalize();
    let light_color = Vec3::new(1., 1., 1.);
    let light_intensity = 1.;

    let bg_light_color = Vec3::new(1., 1., 1.);
    let bg_light_intensity = 0.1;

    let shadow_dist_min = 0.0;
    let shadow_dist_max = max_distance;
    let shadow_fuzziness = 5.;

    let ao_step_size = 0.05;
    let ao_intensity = 0.3;
    let ao_iterations = 3;


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

        shadow_dist_min,
        shadow_dist_max,
        shadow_fuzziness,

        ao_step_size,
        ao_intensity,
        ao_iterations,
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
            self.distance_field(&(p + self.offset_x)) - self.distance_field(&(p - self.offset_x)),
            self.distance_field(&(p + self.offset_y)) - self.distance_field(&(p - self.offset_y)),
            self.distance_field(&(p + self.offset_z)) - self.distance_field(&(p - self.offset_z)),
        ).normalize()
    }

    fn shading(&self, p: &Vec3) -> Vec4 {
        let n = self.get_normal(&p);
        let shadow = self.shadow(&p, &n);
        let ambient_occlusion = self.ambient_occlusion(&p, &n);

        // Vec4::from_vec3(&n, 1.)

        let sun_light = self.obj_color * (self.light_color * Vec3::dot(&(-self.light_dir), &n).clamp(0., 1.) * self.light_intensity * shadow);
        let bg_light = self.obj_color * (self.bg_light_color * self.bg_light_intensity) * ambient_occlusion;

        let light = sun_light + bg_light;

        Vec4::from_vec3(&light, 1.)
    }

    fn shadow(&self, p: &Vec3, n: &Vec3) -> f64 {
        let sro = p + n * self.accuracy;
        let sr = Ray::new(&sro, &(-self.light_dir));

        let mut t: f64 = self.shadow_dist_min;
        let mut result: f64 = 1.0;

        while t < self.shadow_dist_max {
            let p = sr.orig + sr.dir * t;
            let d = self.distance_field(&p);

            if d < self.accuracy {
                return 0.;
            }

            result = result.min(self.shadow_fuzziness * d / t);
            t += d;
        }

        return result.clamp(0., 1.);
    }

    fn ambient_occlusion(&self, p: &Vec3, n: &Vec3) -> f64 {
        let mut ao: f64 = 0.0;
        let mut dist: f64;

        for i in 0..self.ao_iterations {
            dist = self.ao_step_size * (i + 1) as f64;
            let point = p + &(n * dist);

            ao += f64::max(0.0, (dist - self.distance_field(&point)) / dist);
        }

        (1.0 - ao * self.ao_intensity).clamp(0., 1.)
    }
}

