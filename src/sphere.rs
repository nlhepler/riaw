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
        let a = r.direction.squared_len();
        let b = oc.dot(&r.direction);
        let c = oc.squared_len() - self.radius * self.radius;
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

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f32,
    pub time1: f32,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Box<dyn Material>,
    ) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        return self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0);
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord<'_>> {
        let oc = r.origin - self.center(r.time);
        let a = r.direction.squared_len();
        let b = oc.dot(&r.direction);
        let c = oc.squared_len() - self.radius * self.radius;
        let d = b * b - a * c;
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
                let normal = p - self.center(r.time) / self.radius;
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
