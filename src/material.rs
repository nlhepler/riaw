use rand::Rng;

use crate::prelude::{
    random_in_unit_sphere, reflect, refract, schlick, vec3, HitRecord, Ray, Vec3,
};

pub trait Material: Sync + Send {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Box<Self> {
        Box::new(Lambertian { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Box<Self> {
        Box::new(Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.direction.as_unit(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        if scattered.direction.dot(&hit.normal) > 0.0 {
            return Some((self.albedo, scattered));
        }
        None
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Box<Self> {
        Box::new(Dielectric { refractive_index })
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(r.direction, hit.normal);
        let attenuation = vec3![1.0, 1.0, 1.0];
        let (outward_normal, ni_over_nt, cosine) = if r.direction.dot(&hit.normal) > 0.0 {
            let cosine = self.refractive_index * r.direction.dot(&hit.normal) / r.direction.len();
            (-hit.normal, self.refractive_index, cosine)
        } else {
            let cosine = -(r.direction.dot(&hit.normal)) / r.direction.len();
            (hit.normal, 1.0 / self.refractive_index, cosine)
        };

        let scattered = if let Some(refracted) = refract(r.direction, outward_normal, ni_over_nt) {
            let mut rng = rand::thread_rng();
            let reflect_prob = schlick(cosine, self.refractive_index);
            if rng.gen::<f32>() < reflect_prob {
                reflected
            } else {
                refracted
            }
        } else {
            reflected
        };

        Some((attenuation, Ray::new(hit.p, scattered)))
    }
}
