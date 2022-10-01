use rayon::prelude::*;

use crate::{
    camera::Camera,
    color::write_color,
    hittable::Hittable,
    hittable_list::HittableList,
    material::Material,
    vec3::{Color, Vec3},
};

use std::f64::INFINITY;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;

mod vec3;

fn random_scene() -> HittableList {
    let mut world = HittableList::new(Vec::new());
    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.add(Hittable::Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rtweekend::random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * rtweekend::random_double(),
                0.2,
                b as f64 + 0.9 * rtweekend::random_double(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Material::Lambertian { albedo }
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rtweekend::random_double_range(0.0, 0.5);
                    Material::Metal { albedo, fuzz }
                } else {
                    // glass
                    Material::Dielectric { ir: 1.5 }
                };
                world.add(Hittable::Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                });
            }
        }
    }
    let material1 = Material::Dielectric { ir: 1.5 };
    world.add(Hittable::Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    let material2 = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    world.add(Hittable::new_sphere(Vec3::new(-4, 1, 0), 1.0, material2));
    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.add(Hittable::new_sphere(Vec3::new(4, 1, 0), 1.0, material3));
    world
}

fn ray_color(r: &ray::Ray, world: &HittableList, depth: i32) -> Color {
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
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    // World

    // let material_ground = Material::Lambertian {
    //     albedo: Color::new(0.8, 0.8, 0.0),
    // };
    // let material_center = Material::Lambertian {
    //     albedo: Color::new(0.1, 0.2, 0.5),
    // };
    // let material_center = Material::Dielectric { ir: 1.5 };
    // let material_left = Material::Dielectric { ir: 1.5 };
    // let material_right = Material::Metal {
    //     albedo: Color::new(0.8, 0.6, 0.2),
    //     fuzz: 0.0,
    // };
    // let to_add = vec![
    //     Hittable::new_sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground),
    //     Hittable::new_sphere(Vec3::new(0, 0, -1), 0.5, material_center),
    //     Hittable::new_sphere(Vec3::new(-1, 0, -1), 0.5, material_left),
    //     // Hittable::new_sphere(Vec3::new(-1, 0, -1), -0.4, material_left),
    //     Hittable::new_sphere(Vec3::new(1, 0, -1), 0.5, material_right),
    // ];
    // let world = HittableList::new(to_add);

    let world = random_scene();

    // Camera
    const MAX_DEPTH: i32 = 50;

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    (0..IMAGE_HEIGHT).rev().for_each(|j| {
        eprintln!("Scanlines remaining: {}", j);
        let pixel_colors: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                (0..SAMPLES_PER_PIXEL)
                    .map(|_| {
                        let u = (i as f64 + rtweekend::random_double()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + rtweekend::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                        let r = cam.get_ray(u, v);
                        ray_color(&r, &world, MAX_DEPTH)
                    })
                    .sum()
            })
            .collect();
        pixel_colors
            .iter()
            .for_each(|pixel_color| write_color(*pixel_color, SAMPLES_PER_PIXEL));
    });
    eprintln!("Done");
}
