mod vec3;
use std::fs::File;
use std::io::prelude::*;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("cool.ppm")?;
    let nx = 200;
    let ny = 100;
    write!(file, "P3\n{} {}\n255\n", nx, ny)?;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2_f32;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    let mut cool = Vec3 {
        x: 1.,
        y: 3.,
        z: 4.,
    };

    Ok(())
}
