#![feature(const_trait_impl)]
#![feature(const_fn_floating_point_arithmetic)]
use std::f64::INFINITY;

use crate::{
    camera::Camera,
    color::write_color,
    hittable_list::HittableList,
    material::Material,
    sphere::Sphere,
    vec3::{Color, Vec3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn ray_color(r: &ray::Ray, world: &mut HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(new_hit_record) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = Material::scatter(r, &new_hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    // World
    let mut world = HittableList::default();
    let material_ground = Material::Lambertian {
        albedo: (Color::new(0.8, 0.8, 0.0)),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dielectric { ir: 1.5 };
    let material_right = Material::Metal {
        albedo: (Color::new(0.8, 0.6, 0.2)),
        fuzz: 0.0,
    };
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    // Camera
    // const VIEWPORT_HEIGHT: f64 = 2.0;
    // const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    // const FOCAL_LENGTH: f64 = 1.0;
    // const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    // const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    // const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    // const LOWER_LEFT_CORNER: Vec3 =
    //     ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
    const MAX_DEPTH: i32 = 50;
    let cam = Camera::new();
    // Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (i as f64 + rtweekend::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rtweekend::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &mut world, MAX_DEPTH)
                })
                .sum();

            write_color(pixel_color, SAMPLES_PER_PIXEL)
        }
    }
    eprintln!("Done");
}
