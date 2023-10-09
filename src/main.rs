use std::io::Write;
use std::time::Instant;
use std::{mem, thread};
use std::ops::Deref;
use std::sync::{Arc};
use std::thread::JoinHandle;

use image::{Rgb, RgbImage};

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
const IMAGE_WIDTH: u32 = 1080 * 2;
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

    let horizontal = Vec3::cross(&view_direction, &Vec3::new(0., 1., 0.)).normalize() * viewport_width;
    let vertical = -Vec3::cross(&view_direction, &horizontal).normalize() * viewport_height;

    let lower_left_view_corner = -horizontal / 2.0 - vertical / 2.0;
    let lower_left_corner = origin + lower_left_view_corner + view_direction * focal_length;

    dbg!(lower_left_corner);
    dbg!(lower_left_corner + horizontal * 1. + vertical * 1.);

    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let timer_start = Instant::now();

    let mut rows: [Option<JoinHandle<[[u8; 3]; IMAGE_WIDTH as usize]>>; IMAGE_HEIGHT as usize] =  unsafe {
        mem::MaybeUninit::uninit().assume_init()
    };

    let arc_ray_marcher = Arc::new(ray_marcher);
    let arc_origin = Arc::new(origin);
    let arc_horizontal = Arc::new(horizontal);

    for j in 0..IMAGE_HEIGHT {
        let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
        let pixel_pos_row = lower_left_corner + vertical * v;


        rows[j as usize] = Some(calc_row(arc_ray_marcher.clone(), arc_origin.clone(), arc_horizontal.clone(), pixel_pos_row));
    }

    for j in 0..IMAGE_HEIGHT {
        let row: Option<[[u8; 3]; IMAGE_WIDTH as usize]> = rows.get_mut(j as usize)
            .map(|x| -> [[u8; 3]; IMAGE_WIDTH as usize] {
                let handle = x.take().unwrap();
                let result = handle.join();
                result.unwrap()
            });

        for i in 0..IMAGE_WIDTH {
            row.map(|x| -> () {
                image.put_pixel(i, IMAGE_HEIGHT - j - 1, Rgb(x[i as usize]));
            });
        }
    }




    // for j in 0..IMAGE_HEIGHT {
    //     for i in 0..IMAGE_WIDTH {
    //         let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
    //         let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
    //
    //         let pixel_pos = lower_left_corner + horizontal * u + vertical * v;
    //
    //         let r = Ray::new(origin, pixel_pos - origin);
    //         let pixel_color = ray_marcher.ray_marching(r);
    //
    //         image.put_pixel(i, IMAGE_HEIGHT - j - 1, Rgb(pixel_color.to_pixel_data()));
    //     }
    // }

    let timer_duration = timer_start.elapsed();

    println!("Rendered image ({IMAGE_WIDTH}x{IMAGE_HEIGHT}) in {:?}", timer_duration);

    image
}

fn calc_row(rm: Arc<RayMarcher>, origin: Arc<Vec3>, pixel_pos_horizontal: Arc<Vec3>, pixel_pos_row: Vec3) -> JoinHandle<[[u8; 3]; IMAGE_WIDTH as usize]> {
    thread::spawn(move || {
        let mut row: [[u8; 3]; IMAGE_WIDTH as usize] = unsafe {
            mem::MaybeUninit::uninit().assume_init()
        };

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);

            let pixel_pos = pixel_pos_row + pixel_pos_horizontal.deref() * u;

            let r = Ray::new(origin.deref(), &(pixel_pos - origin.deref()));
            let pixel_color = rm.ray_marching(r);

            row[i as usize] = pixel_color.to_pixel_data();
        }

        row
    })
}
