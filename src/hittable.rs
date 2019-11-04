use crate::prelude::{Material, Ray, Vec3};

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        HitRecord {
            t,
            p,
            normal,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord<'_>>;
    fn into_box(self) -> Box<dyn Hittable + Sync>;
}

impl Hittable for Vec<Box<dyn Hittable + Sync>> {
    fn hit(&self, r: &Ray, tmin: f32, mut tmax: f32) -> Option<HitRecord<'_>> {
        let mut result = None;
        for h in self.iter() {
            if let Some(hit) = h.hit(r, tmin, tmax) {
                tmax = hit.t;
                result = Some(hit);
            }
        }
        result
    }

    fn into_box(self) -> Box<dyn Hittable + Sync> {
        Box::new(self) as Box<dyn Hittable + Sync>
    }
}
