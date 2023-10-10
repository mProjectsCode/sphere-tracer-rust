use std::{mem, thread};
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;

use image::{Rgb, RgbImage};
use itertools::{iproduct, Itertools};
use rayon::prelude::*;

use distance_fields::DistanceField;
use distance_fields::DistanceFunction;
use ray::Ray;
use ray_marching::create_ray_marcher;
use ray_marching::RayMarcher;
use vec3::Vec3;
use vec4::Vec4;

#[path = "math/vec3.rs"]
mod vec3;
#[path = "math/vec4.rs"]
mod vec4;
#[path = "math/ray.rs"]
mod ray;
#[path = "ray_marching.rs"]
mod ray_marching;
#[path = "distance_fields.rs"]
mod distance_fields;

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO as f32) as u32;

fn main() {
    let sphere: DistanceField = DistanceField::Sphere(distance_fields::Sphere {
        pos: Vec3::new(0., 0., -2.),
        size: 0.2,
    });
    // // let cuboid: DistanceField = DistanceField::Cuboid(distance_fields::Cuboid {
    // //     pos: Vec3::new(0.8, 0., -2.),
    // //     size: Vec3::new(0.5, 0.7, 0.5),
    // // });
    //
    // let sphere2: DistanceField = DistanceField::Sphere(distance_fields::Sphere {
    //     pos: Vec3::new(0.8, 0., -2.),
    //     size: 0.6,
    // });
    // let sdf: DistanceField = DistanceField::Union(Box::from(distance_fields::Union {
    //     a: sphere,
    //     b: sphere2,
    // }));

    let julia = DistanceField::Julia(distance_fields::Julia {
        pos: Vec3::new(0., 0., -1.),
        iterations: 2000,
        // -5, 2, 4, -2

        c: Vec4::new(-1.51, 5.9, 4., -2.) / 10.,
        traps: false,
        cut: true,
        cut_y: 0.,
    });

    let ray_marcher = create_ray_marcher(julia);

    let image = create_image(ray_marcher);

    image.save("out.png").unwrap();

    println!("Vec size {}", mem::size_of::<Vec3>());
}

fn create_image(ray_marcher: RayMarcher) -> RgbImage {
    // Camera
    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 3.;

    let origin = Vec3::new(-0.42, 0.05, -0.7);
    let looking_at = Vec3::new(0.3, -1.6, -2.5);
    let view_direction = (looking_at - origin).normalize();

    // horizontal and vertical vector of the view port
    let horizontal = Vec3::cross(&view_direction, &Vec3::new(0., 1., 0.)).normalize() * viewport_width;
    let vertical = -Vec3::cross(&view_direction, &horizontal).normalize() * viewport_height;

    // lower left corner of the view port
    let ll_view_corner = -horizontal / 2.0 - vertical / 2.0;
    // ray direction of the lower left viewport corner
    let ll_corner = ll_view_corner + view_direction * focal_length;

    // dbg!(ll_corner);
    // dbg!(ll_corner + horizontal * 1. + vertical * 1.);

    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let timer_start = Instant::now();

    let arc_ray_marcher = Arc::new(ray_marcher);

    // iterate over the pixel rows
    let pixel_data: Vec<Vec<[u8; 3]>> = (0..IMAGE_HEIGHT).into_par_iter().map(|j| -> Vec<[u8; 3]> {
        // clone a bunch of stuff into this scope
        let clone_ray_marcher = arc_ray_marcher.clone();
        let clone_origin = origin.clone();
        let clone_ll_corner = ll_corner.clone();
        let clone_horizontal = horizontal.clone();
        let clone_vertical = vertical.clone();

        // iterate over the pixels in the row and calculate their color
        (0..IMAGE_WIDTH).map(|i| -> [u8; 3] {
            calc_pixel(clone_ray_marcher.deref(), i, j, &clone_origin, &clone_ll_corner, &clone_horizontal, &clone_vertical)
        }).collect()
    }).collect();

    let timer_duration = timer_start.elapsed();

    // set the pixel in the actual image
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Rgb(pixel_data[j as usize][i as usize]);

            image.put_pixel(i, IMAGE_HEIGHT - j - 1, pixel_color);
        }
    }

    println!("Rendered image ({IMAGE_WIDTH}x{IMAGE_HEIGHT}) in {:?}", timer_duration);

    image
}

fn calc_pixel(rm: &RayMarcher, i: u32, j: u32, origin: &Vec3, ll_corner: &Vec3, horizontal: &Vec3, vertical: &Vec3) -> [u8; 3] {
    let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
    let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

    let pixel_pos = ll_corner + horizontal * u + vertical * v;

    let r = Ray::new(&origin, &pixel_pos);
    let pixel_color = rm.ray_marching(r);

    pixel_color.to_pixel_data()
}