mod camera;
mod hittable;
mod material;
mod ray;
mod rng;
mod sphere;
mod tracer;
mod vec3;

pub mod random_spheres;

use rng::Rng;
use vec3::Vec3;

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rng::thread_rng();
    loop {
        let p = 2.0 * vec3![rng.gen::<f32>(), rng.gen::<f32>(), 0] - vec3![1, 1, 0];
        if p.squared_len() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rng::thread_rng();
    loop {
        let p = 2.0 * vec3![rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()] - Vec3::ones();
        if p.squared_len() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.as_unit();
    let dt = uv.dot(&n);
    let d = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if d > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * d.sqrt())
    } else {
        None
    }
}

pub fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub mod prelude {
    pub use super::camera::Camera;
    pub use super::hittable::{HitRecord, Hittable};
    pub use super::material::{Dielectric, Lambertian, Material, Metal};
    pub use super::ray::Ray;
    pub use super::rng::{thread_rng, Rng};
    pub use super::sphere::Sphere;
    pub use super::tracer::Tracer;
    pub use super::vec3;
    pub use super::vec3::Vec3;
    pub use super::{random_in_unit_disk, random_in_unit_sphere, reflect, refract, schlick};
}
