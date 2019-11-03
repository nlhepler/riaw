use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use failure::Error;
use rand::Rng;
use rayon::prelude::*;

use riaw::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;
const SAMPLES: usize = 100;

fn color<'a, T>(r: &Ray, world: &'a Vec<T>, depth: usize) -> Vec3
where
    &'a T: Hittable<'a>,
{
    if let Some(hit) = world.iter().hit(r, 1e-3, std::f32::MAX) {
        if depth >= 50 {
            return Vec3::zeros();
        }
        return if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            attenuation * color(&scattered, world, depth + 1)
        } else {
            Vec3::zeros()
        };
    }
    let unit_direction = r.direction.as_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::ones() + t * vec3![0.5, 0.7, 1.0]
}

fn scene() -> Vec<Sphere> {
    let mut rng = rand::thread_rng();
    let mut randf = move || rng.gen::<f32>();
    let mut result = vec![Sphere::new(
        vec3![0, -1000, 0],
        1000.0,
        Lambertian::new(vec3![0.5, 0.5, 0.5]),
    )];
    for a in -11..11 {
        for b in -11..11 {
            let mat = randf();
            let center = vec3![a as f32 + 0.9 * randf(), 0.2, b as f32 + 0.9 * randf()];
            if (center - vec3![4, 0.2, 0]).len() > 0.9 {
                if mat < 0.8 {
                    result.push(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(vec3![
                            randf() * randf(),
                            randf() * randf(),
                            randf() * randf()
                        ]),
                    ));
                } else if mat < 0.95 {
                    result.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            vec3![
                                0.5 * (1.0 + randf()),
                                0.5 * (1.0 + randf()),
                                0.5 * (1.0 + randf())
                            ],
                            0.5 * randf(),
                        ),
                    ));
                } else {
                    result.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    result.push(Sphere::new(vec3![0, 1, 0], 1.0, Dielectric::new(1.5)));
    result.push(Sphere::new(
        vec3![-4, 1, 0],
        1.0,
        Lambertian::new(vec3![0.4, 0.2, 0.1]),
    ));
    result.push(Sphere::new(
        vec3![4, 1, 0],
        1.0,
        Metal::new(vec3![0.7, 0.6, 0.5], 0.0),
    ));

    result
}

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let prog = args.next().unwrap();
    if args.len() != 1 {
        println!(
            "Usage: {} <output-path>",
            Path::new(&prog).file_name().unwrap().to_str().unwrap()
        );
        return Ok(());
    }
    let path = args.next().unwrap();
    let file = File::create(&path)?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut stream = writer.stream_writer();

    let look_from = vec3![13, 2, 3];
    let look_at = vec3![0, 0, 0];
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        vec3![0, 1, 0],
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        aperature,
        dist_to_focus,
    );

    /*
    let world = vec![
        Sphere::new(vec3![0, 0, -1], 0.5, Lambertian::new(vec3![0.1, 0.2, 0.5])),
        Sphere::new(
            vec3![0, -100.5, -1],
            100.0,
            Lambertian::new(vec3![0.8, 0.8, 0.0]),
        ),
        Sphere::new(vec3![1, 0, -1], 0.5, Metal::new(vec3![0.8, 0.6, 0.2], 0.05)),
        Sphere::new(vec3![-1, 0, -1], 0.5, Dielectric::new(1.5)),
        Sphere::new(vec3![-1, 0, -1], -0.4, Dielectric::new(1.5)),
    ];
    */
    let world = scene();

    let data = (0..HEIGHT)
        .into_par_iter()
        .rev()
        .map(|y| {
            let mut block = vec![0u8; 3 * WIDTH as usize];
            (0..WIDTH as usize).for_each(|x| {
                let mut rng = rand::thread_rng();
                let mut avg = Vec3::zeros();
                for _ in 0..SAMPLES {
                    let v = (y as f32 + rng.gen::<f32>()) / HEIGHT as f32;
                    let u = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
                    let r = cam.get_ray(u, v);
                    avg += color(&r, &world, 0);
                }
                avg /= SAMPLES as f32;
                block[3 * x..3 * x + 3].copy_from_slice(&avg.sqrt().to_rgb()[..]);
            });
            block
        })
        .collect::<Vec<_>>();

    for pix in data {
        stream.write(pix.as_ref())?;
    }
    stream.finish()?;

    Ok(())
}
