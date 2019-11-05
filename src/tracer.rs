use rayon::prelude::*;

use crate::prelude::{thread_rng, Camera, Hittable, Ray, Rng, Vec3, BVH};

const MAX_DEPTH: usize = 50;

pub fn color<F>(r: &Ray, world: &BVH, skybox: F, depth: usize) -> Vec3
where
    F: Fn(&Ray) -> Vec3,
{
    if let Some(hit) = world.hit(r, 1e-3, std::f32::MAX) {
        if depth >= MAX_DEPTH {
            return Vec3::zeros();
        }
        return if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            attenuation * color(&scattered, world, skybox, depth + 1)
        } else {
            Vec3::zeros()
        };
    }
    skybox(r)
}

fn split_scanlines<'a>(
    mut buffer: &'a mut [u32],
    width: usize,
    height: usize,
) -> Vec<(usize, &'a mut [u32])> {
    if buffer.len() < width * height {
        panic!("rendering buffer is insufficiently sized");
    }
    buffer = &mut buffer[0..width * height];
    let mut res = vec![];
    for y in (0..height).rev() {
        if buffer.len() <= width {
            res.push((y, buffer));
            break;
        }
        let (chnk, rest) = buffer.split_at_mut(width);
        res.push((y, chnk));
        buffer = rest;
    }
    res
}

pub struct Tracer<F: Fn(&Ray) -> Vec3 + Sync> {
    camera: Camera,
    world: BVH,
    skybox: F,
}

impl<F: Fn(&Ray) -> Vec3 + Sync> Tracer<F> {
    pub fn new(camera: Camera, world: Vec<Box<dyn Hittable + Sync>>, skybox: F) -> Self {
        let world = BVH::new(world, camera.time0, camera.time1);
        Tracer {
            camera,
            world,
            skybox,
        }
    }

    pub fn render_sample(
        &self,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        n_samples: usize,
    ) -> usize {
        split_scanlines(buffer, width, height)
            .into_par_iter()
            .for_each(|(y, block)| {
                let mut rng = thread_rng();
                (0..width).for_each(|x| {
                    let mut avg = Vec3::from_argb(block[x]).powi(2) * n_samples as f32;
                    let v = (y as f32 + rng.gen::<f32>()) / height as f32;
                    let u = (x as f32 + rng.gen::<f32>()) / width as f32;
                    let r = self.camera.get_ray(u, v);
                    avg += color(&r, &self.world, &self.skybox, 0);
                    avg /= (n_samples + 1) as f32;
                    block[x] = avg.sqrt().to_argb();
                });
            });
        n_samples + 1
    }
}
