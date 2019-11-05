use crate::prelude::{vec3, HitRecord, Hittable, Material, Ray, Vec3, AABB};

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

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - vec3![self.radius, self.radius, self.radius],
            self.center + vec3![self.radius, self.radius, self.radius],
        ))
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
        let slope = self.center1 - self.center0;
        let x = (time.min(self.time1) - self.time0) / (self.time1 - self.time0);
        self.center0 + slope * x
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
                let normal = (p - self.center(r.time)) / self.radius;
                HitRecord::new(t, p, normal, self.material.as_ref())
            })
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.center(t0) - vec3![self.radius, self.radius, self.radius],
            self.center(t0) + vec3![self.radius, self.radius, self.radius],
        );
        let box1 = AABB::new(
            self.center(t1) - vec3![self.radius, self.radius, self.radius],
            self.center(t1) + vec3![self.radius, self.radius, self.radius],
        );
        Some(AABB::surrounding_box(box0, box1))
    }

    fn into_box(self) -> Box<dyn Hittable + Sync> {
        Box::new(self) as Box<dyn Hittable + Sync>
    }
}
