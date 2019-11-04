use std::path::Path;

use failure::Error;
use minifb::{Key, Window, WindowOptions};

use riaw::prelude::*;
use riaw::random_spheres::{skybox, world};

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let prog = args.next().unwrap();
    if args.len() != 0 {
        println!(
            "Usage: {}",
            Path::new(&prog).file_name().unwrap().to_str().unwrap()
        );
        return Ok(());
    }

    let look_from = vec3![13, 2, 3];
    let look_at = vec3![0, 0, 0];
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vec3![0, 1, 0],
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        aperature,
        dist_to_focus,
    );
    let scene = world();
    let sampler = Tracer::new(camera, scene, skybox);

    let mut window = Window::new(
        "riaw - frame 0",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )?;
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut n_samples = 0;

    loop {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }
        n_samples = sampler.render_sample(&mut buffer, WIDTH, HEIGHT, n_samples);
        window.update_with_buffer(buffer.as_ref())?;
        window.set_title(format!("riaw - frame {}", n_samples).as_str());
    }

    Ok(())
}
