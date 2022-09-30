use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold(
                (t_max, None),
                |(closest_so_far, returnable_rec), next_object| match next_object.hit(
                    ray,
                    t_min,
                    closest_so_far,
                ) {
                    Some(hit) => (hit.t, Some(hit)),
                    None => (closest_so_far, returnable_rec),
                },
            )
            .1
    }
}
