use crate::prelude::{thread_rng, HitRecord, Hittable, Ray, Rng, AABB};

pub enum BVH {
    Node {
        left: Box<dyn Hittable + Sync>,
        right: Box<dyn Hittable + Sync>,
        bbox: AABB,
    },
    Leaf {
        obj: Box<dyn Hittable + Sync>,
        bbox: AABB,
    },
}

macro_rules! box_compare {
    ($name:ident, $ax:ident) => {
        fn $name(
            lhs: &Box<dyn Hittable + Sync>,
            rhs: &Box<dyn Hittable + Sync>,
        ) -> std::cmp::Ordering {
            let box_lhs = lhs.bounding_box(0.0, 0.0).unwrap();
            let box_rhs = rhs.bounding_box(0.0, 0.0).unwrap();
            box_lhs
                .min
                .$ax
                .partial_cmp(&box_rhs.min.$ax)
                .expect("NaN encountered in BVH construction")
        }
    };
}

box_compare!(box_compare_x, x);
box_compare!(box_compare_y, y);
box_compare!(box_compare_z, z);

impl BVH {
    pub fn new(mut objs: Vec<Box<dyn Hittable + Sync>>, t0: f32, t1: f32) -> Self {
        match thread_rng().gen_range(0usize, 3) {
            0 => objs.sort_by(box_compare_x),
            1 => objs.sort_by(box_compare_y),
            2 => objs.sort_by(box_compare_z),
            _ => panic!("unreachable"),
        };

        if objs.len() == 1 {
            let obj = objs.pop().unwrap();
            let bbox = obj.bounding_box(t0, t1).unwrap();
            BVH::Leaf { obj, bbox }
        } else if objs.len() == 2 {
            let right = objs.pop().unwrap();
            let left = objs.pop().unwrap();
            let bbox = AABB::surrounding_box(
                left.bounding_box(t0, t1).unwrap(),
                right.bounding_box(t0, t1).unwrap(),
            );
            BVH::Node { left, right, bbox }
        } else {
            let right =
                BVH::new(objs.drain(objs.len() / 2..).collect::<Vec<_>>(), t0, t1).into_box();
            let left = BVH::new(objs, t0, t1).into_box();
            let bbox = AABB::surrounding_box(
                left.bounding_box(t0, t1).unwrap(),
                right.bounding_box(t0, t1).unwrap(),
            );
            BVH::Node { left, right, bbox }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord<'_>> {
        match self {
            BVH::Leaf { obj, bbox } => {
                if bbox.hit(r, tmin, tmax) {
                    obj.hit(r, tmin, tmax)
                } else {
                    None
                }
            }
            BVH::Node { left, right, bbox } => {
                if bbox.hit(r, tmin, tmax) {
                    let hit_left = left.hit(r, tmin, tmax);
                    let hit_right = right.hit(r, tmin, tmax);
                    match (hit_left, hit_right) {
                        (Some(left), Some(right)) => {
                            if left.t < right.t {
                                Some(left)
                            } else {
                                Some(right)
                            }
                        }
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        match self {
            BVH::Node { bbox, .. } => Some(*bbox),
            BVH::Leaf { bbox, .. } => Some(*bbox),
        }
    }

    fn into_box(self) -> Box<dyn Hittable + Sync> {
        Box::new(self) as Box<dyn Hittable + Sync>
    }
}
