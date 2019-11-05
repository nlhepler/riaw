use std::path::Path;

use failure::Error;
use log::info;
use minifb::{Key, Scale, Window, WindowOptions};

use riaw::prelude::*;
use riaw::scenes::random_spheres::{skybox, world};

const WIDTH: usize = 1200;
const HEIGHT: usize = 600;

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut args = std::env::args();
    let prog = args.next().unwrap();
    if args.len() != 0 {
        println!(
            "Usage: {}",
            Path::new(&prog).file_name().unwrap().to_str().unwrap()
        );
        return Ok(());
    }

    let randf = || thread_rng().gen_range(-1f32, 1f32);
    let look_from = 18.38 * vec3![randf(), randf().abs(), randf()].as_unit();
    info!("look_from: {:?}", look_from);
    let look_at = vec3![0, 0, 0];
    let dist_to_focus = 10.0;
    let aperature = 0.0;
    let camera = Camera::new(
        look_from,
        look_at,
        vec3![0, 1, 0],
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        aperature,
        dist_to_focus,
        0.0,
        1.0,
    );
    let scene = world();
    let sampler = Tracer::new(camera, scene, skybox);

    let mut window = Window::new(
        "riaw - frame 0",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: Scale::HiDPI,
            ..WindowOptions::default()
        },
    )?;
    let mut buffer = vec![0u32; WIDTH * HEIGHT * 4];
    let mut n_samples = 0;

    loop {
        if !window.is_open()
            || window.is_key_down(Key::Escape)
            || window.is_key_down(Key::Q)
            || window.is_key_down(Key::S)
            || n_samples >= 512
        {
            info!("tracing halted");
            break;
        }
        n_samples = sampler.render_sample(&mut buffer, WIDTH * 2, HEIGHT * 2, n_samples);
        window.update_with_buffer(buffer.as_ref())?;
        window.set_title(format!("riaw - frame {}", n_samples).as_str());
    }

    while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        window.update();
    }

    Ok(())
}
