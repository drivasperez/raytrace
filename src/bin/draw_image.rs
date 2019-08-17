extern crate raytrace;
use self::raytrace::*;
use rand::Rng;
use raytrace::camera;
use raytrace::vec3::Vec3;

use image;

fn main() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let nx = 50;
    let ny = 50;
    let ns = 30;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let world = random_scene();

    let cam = camera::Camera::new(
        Vec3::new(3.0, 3.0, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        nx as f32 / ny as f32,
    );

    println!("Beginning to draw.");

    imgbuf.enumerate_pixels_mut().for_each(|(i, j, pixel)| {
        let mut col = Vec3::new(0., 0., 0.);
        (0..ns).for_each(|_| {
            let randi: f32 = rng.gen();
            let randj: f32 = rng.gen();
            let u = (i as f32 + randi) / nx as f32;
            let v = ((ny - j) as f32 + randj) / ny as f32;
            let r = cam.get_ray(u, v);
            col += colour(r, &world, 0);
        });
        col /= ns as f32;
        col = Vec3 {
            x: col.x.sqrt(),
            y: col.y.sqrt(),
            z: col.z.sqrt(),
        };

        let ir = (255.99 * col.x) as u8;
        let ig = (255.99 * col.y) as u8;
        let ib = (255.99 * col.z) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    });

    imgbuf.save("newcool.png")?;

    println!("Finished drawing!");

    Ok(())
}
