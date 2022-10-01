use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn get_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        };
        (front_face, normal)
    }
}
pub enum Hittable {
    Sphere {
        center: Point3,
        radius: f64,
        material: Material,
    },
}
impl Hittable {
    pub fn new_sphere(center: Point3, radius: f64, material: Material) -> Hittable {
        Hittable::Sphere {
            center,
            radius,
            material,
        }
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Hittable::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = ray.origin - *center;
                let a = ray.direction.squared_length();
                let half_b = oc.dot(&ray.direction);
                let c = oc.squared_length() - radius * radius;
                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    return None;
                }
                // Find the nearest root that lies in the acceptable range.
                let sqrtd = discriminant.sqrt();
                let root = (-half_b - sqrtd) / a;
                let new_root = if root < t_min || t_max < root {
                    (-half_b + sqrtd) / a
                } else {
                    root
                };
                if new_root < t_min || t_max < new_root {
                    return None;
                }

                let outward_normal = (ray.at(new_root) - *center) / *radius;
                let (front_face, normal) = HitRecord::get_face_normal(ray, &outward_normal);

                Some(HitRecord {
                    t: new_root,
                    p: ray.at(new_root),
                    normal,
                    material: *material,
                    front_face,
                })
            }
        }
    }
}
