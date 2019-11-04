use crate::prelude::{
    thread_rng, vec3, Dielectric, Hittable, Lambertian, Metal, Ray, Rng, Sphere, Vec3,
};

pub fn skybox(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.as_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::ones() + t * vec3![0.5, 0.7, 1.0]
}

pub fn world() -> Vec<Box<dyn Hittable + Sync>> {
    let mut rng = thread_rng();
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

    result.into_iter().map(|x| x.into_box()).collect::<Vec<_>>()
}
