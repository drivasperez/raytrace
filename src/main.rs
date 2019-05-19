mod camera;
mod hitable;
mod matter;
mod ray;
mod vec3;
use hitable::Hitable;
use image;
use matter::{Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let world = vec![
        hitable::Sphere::new(
            Vec3::new(0.0, 0.0, -1.),
            0.5,
            Box::new(Lambertian::new(0.8, 0.3, 0.3)),
        ),
        hitable::Sphere::new(
            Vec3::new(0.0, -100.5, -1.),
            100.,
            Box::new(Lambertian::new(0.8, 0.8, 0.8)),
        ),
        hitable::Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.8, 0.6, 0.2)),
        ),
        hitable::Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(Metal::new(0.8, 0.8, 0.8)),
        ),
    ];
    let cam = camera::Camera::default();

    imgbuf.enumerate_pixels_mut().for_each(|(i, j, pixel)| {
        let mut col = Vec3::new(0., 0., 0.);
        (0..ns).for_each(|_| {
            let randi: f32 = rng.gen();
            let randj: f32 = rng.gen();
            let u = (i as f32 + randi) / nx as f32;
            let v = ((ny - j) as f32 + randj) / ny as f32;
            let r = cam.get_ray(u, v);
            let _p = r.point_at_parameter(2.0);
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

    Ok(())
}

fn colour<T: hitable::Hitable>(r: Ray, world: &[T], depth: usize) -> Vec3 {
    let mut rec = hitable::HitRecord::default();
    if world.hit(&r, 0.001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50
            && rec
                .mat_ptr
                .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            attenuation * colour(scattered, world, depth + 1)
        } else {
            Vec3::default()
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

// fn colour<T: hitable::Hitable>(r: Ray, world: &[T], depth: usize) -> Vec3 {
//     let mut rec = hitable::HitRecord::default();

//     if world.hit(&r, 0.001, std::f32::MAX, &mut rec) {
//         let mut scattered = Ray::default();
//         let mut attenuation = Vec3::default();

//         if depth < 50
//             && rec
//                 .mat_ptr
//                 .scatter(&r, &rec, &mut attenuation, &mut scattered)
//         {
//             attenuation * colour::<T>(scattered, world, depth + 1)
//         } else {
//             Vec3::new(0.0, 0.0, 0.0)
//         }
//     } else {
//         let unit_direction = Vec3::unit_vector(r.direction());
//         let t = 0.5 * (unit_direction.y + 1.0);
//         (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
//     }
// }

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let mut p = 2.0 * (Vec3::new(rng.gen(), rng.gen(), rng.gen())) - Vec3::new(1., 1., 1.);

    while p.squared_length() >= 1.0 {
        p = 2.0 * (Vec3::new(rng.gen(), rng.gen(), rng.gen())) - Vec3::new(1., 1., 1.);
    }
    p
}
