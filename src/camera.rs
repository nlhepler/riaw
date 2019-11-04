use crate::prelude::{random_in_unit_disk, thread_rng, Ray, Rng, Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Camera {
    /*
    pub fn default() -> Self {
        let origin = Vec3::zeros();
        let lower_left = vec3![-2, -1, -1];
        let horizontal = vec3![4, 0, 0];
        let vertical = vec3![0, 2, 0];
        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }
    */

    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperature: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let lens_radius = aperature / 2.0;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).as_unit();
        let u = vup.cross(&w).as_unit();
        let v = w.cross(&u);
        let lower_left =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + thread_rng().gen::<f32>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        )
    }
}
