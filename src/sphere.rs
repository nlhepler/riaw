use crate::prelude::{HitRecord, Hittable, Material, Ray, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let d = b * b - a * c; // why not 4.0?
        if d > 0.0 {
            let t = {
                let t = (-b - d.sqrt()) / a;
                if tmin < t && t < tmax {
                    Some(t)
                } else {
                    let t = (-b + d.sqrt()) / a;
                    if tmin < t && t < tmax {
                        Some(t)
                    } else {
                        None
                    }
                }
            };
            t.map(|t| {
                let p = r.point_at(t);
                let normal = (p - self.center) / self.radius;
                HitRecord::new(t, p, normal, self.material.as_ref())
            })
        } else {
            None
        }
    }

    fn into_box(self) -> Box<dyn Hittable + Sync> {
        Box::new(self) as Box<dyn Hittable + Sync>
    }
}
