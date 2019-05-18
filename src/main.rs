mod ray;
mod vec3;
use ray::Ray;
use std::fs::File;
use std::io::prelude::*;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("cool.ppm")?;
    let nx = 200;
    let ny = 100;
    write!(file, "P3\n{} {}\n255\n", nx, ny)?;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = colour(r);
            let ir = (255.00 * col.x) as i32;
            let ig = (255.00 * col.y) as i32;
            let ib = (255.00 * col.z) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn colour(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
