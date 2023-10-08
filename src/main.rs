use std::io::Write;
use std::time::Instant;

use image::{Rgb, RgbImage};

use distance_fields::DistanceField;
use distance_fields::DistanceFunction;
use ray::Ray;
use ray_marching::create_ray_marcher;
use ray_marching::RayMarcher;
use vector::Vec3;

#[path = "math/vector.rs"]
mod vector;
#[path = "math/ray.rs"]
mod ray;
#[path = "ray_marching.rs"]
mod ray_marching;
#[path = "distance_fields.rs"]
mod distance_fields;

fn main() {
    let sphere: DistanceField = DistanceField::Sphere(distance_fields::Sphere {
        pos: Vec3::new(0., 0., -2.),
        size: 0.5,
    });
    let cuboid: DistanceField = DistanceField::Cuboid(distance_fields::Cuboid {
        pos: Vec3::new(0.8, 0., -2.),
        size: Vec3::new(0.5, 0.7, 0.5),
    });
    let sdf: DistanceField = DistanceField::Union(Box::from(distance_fields::Union {
        a: sphere,
        b: cuboid,
    }));

    let ray_marcher = create_ray_marcher(sdf);

    let image = create_image(ray_marcher);

    image.save("out.png").unwrap();
}

fn create_image(ray_marcher: RayMarcher) -> RgbImage {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 720;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO as f32) as u32;

    // Camera
    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.5;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    dbg!(lower_left_corner);
    dbg!(lower_left_corner + horizontal * 1. + vertical * 1.);

    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let timer_start = Instant::now();

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let pixel_pos = lower_left_corner + horizontal * u + vertical * v;

            let r = Ray::new(origin, pixel_pos - origin);
            let pixel_color = ray_marcher.ray_marching(r);

            image.put_pixel(i, IMAGE_HEIGHT - j - 1, Rgb(pixel_color.to_pixel_data()));
        }
    }

    let timer_duration = timer_start.elapsed();

    println!("Rendered image ({IMAGE_WIDTH}x{IMAGE_HEIGHT}) in {:?}", timer_duration);

    image
}
