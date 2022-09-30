use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{Color, Vec3},
};
#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { ir: f64 },
}

impl Material {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
    pub fn scatter(ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match rec.material {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
                // Catch degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.p, scatter_direction);
                Some((albedo, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray.direction.unit_vector().reflect(&rec.normal);
                let scattered = Ray::new(rec.p, reflected + fuzz * Vec3::random_in_unit_sphere());
                if scattered.direction.dot(&rec.normal) > 0.0 {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric { ir } => {
                let attenuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / ir } else { ir };
                let unit_direction = ray.direction.unit_vector();
                // Total internal reflection handling
                let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let direction = if (refraction_ratio * sin_theta > 1.0)
                    || (Self::reflectance(cos_theta, refraction_ratio) > random_double())
                {
                    unit_direction.reflect(&rec.normal)
                } else {
                    unit_direction.refract(&rec.normal, refraction_ratio)
                };

                Some((attenuation, Ray::new(rec.p, direction)))
            }
        }
    }
}
