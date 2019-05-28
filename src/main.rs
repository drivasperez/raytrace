mod camera;
mod hitable;
mod matter;
mod ray;
mod vec3;
use hitable::{Hitable, Sphere};
use image;
use matter::Material;
use rand::Rng;
use ray::Ray;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let nx = 1000;
    let ny = 500;
    let ns = 300;
    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    let world = random_scene();

    let cam = camera::Camera::new(
        Vec3::new(3.0, 3.0, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        nx as f32 / ny as f32,
    );

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

    Ok(())
}

fn colour<T: hitable::Hitable>(r: Ray, world: &[T], depth: usize) -> Vec3 {
    world
        .hit(&r, 0.001, std::f32::MAX)
        .and_then(|rec| {
            if let Some(mat) = rec.mat_ptr {
                let (attenuation, scattered, did_scatter) = mat.scatter(&r, &rec);
                if depth < 50 && did_scatter {
                    return Some(attenuation * colour(scattered, world, depth + 1));
                } else {
                    return Some(Vec3::default());
                }
            };
            None
        })
        .unwrap_or_else(|| {
            let unit_direction = Vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        })
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    let mut p = 2.0 * (Vec3::new(rng.gen(), rng.gen(), rng.gen())) - Vec3::new(1., 1., 1.);

    while p.squared_length() >= 1.0 {
        p = 2.0 * (Vec3::new(rng.gen(), rng.gen(), rng.gen())) - Vec3::new(1., 1., 1.);
    }
    p
}

fn random_scene() -> Vec<Sphere> {
    let mut rng = rand::thread_rng();
    let mut hit_list = Vec::new();
    hit_list.push(Sphere::new(
        Vec3::new(0.0, -1000., 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let (x, y) = (a as f32, b as f32);
            let shiftx: f32 = rng.gen();
            let shifty: f32 = rng.gen();
            let center = Vec3::new(x + 0.9 * shiftx, 0.2, y + 0.9 * shifty);
            if (center - Vec3::new(4.0, 0.0, 2.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    hit_list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Vec3::new(rng.gen(), rng.gen(), rng.gen())),
                    ))
                } else if choose_mat < 0.95 {
                    hit_list.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(
                            Vec3::new(
                                0.5 * rng.gen::<f32>(),
                                0.5 * rng.gen::<f32>(),
                                0.5 * rng.gen::<f32>(),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    ))
                } else {
                    hit_list.push(Sphere::new(center, 0.2, Material::Dielectric(1.5)))
                }
            };
        }
    }

    hit_list.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(1.5),
    ));
    hit_list.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Vec3::new(0.4, 0.2, 0.1), 0.0),
    ));
    hit_list.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));

    hit_list
}
