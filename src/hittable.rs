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

pub trait Hittable<'a> {
    fn hit(self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord<'a>>;
}

impl<'a, T, U> Hittable<'a> for U
where
    T: Hittable<'a>,
    U: IntoIterator<Item = T>,
{
    fn hit(self, r: &Ray, tmin: f32, mut tmax: f32) -> Option<HitRecord<'a>> {
        let mut result = None;
        for h in self.into_iter() {
            if let Some(hit) = h.hit(r, tmin, tmax) {
                tmax = hit.t;
                result = Some(hit);
            }
        }
        result
    }
}
