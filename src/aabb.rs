use crate::prelude::{vec3, Ray, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

fn fast_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn fast_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> Self {
        let min = vec3![
            fast_min(box0.min.x, box1.min.x),
            fast_min(box0.min.y, box1.min.y),
            fast_min(box0.min.z, box1.min.z)
        ];
        let max = vec3![
            fast_max(box0.max.x, box1.max.x),
            fast_max(box0.max.y, box1.max.y),
            fast_max(box0.max.z, box1.max.z)
        ];
        AABB { min, max }
    }

    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > tmin {
                tmin = t0;
            }
            if t1 < tmax {
                tmax = t1;
            }
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}
