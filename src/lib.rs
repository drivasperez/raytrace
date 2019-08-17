pub mod camera;
pub mod hitable;
pub mod matter;
pub mod ray;
pub mod vec3;
use cfg_if::cfg_if;
use hitable::{Hitable, Sphere};
use matter::Material;
use rand::Rng;
use ray::Ray;
use vec3::Vec3;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }

}

#[wasm_bindgen]
pub struct Pixel(u8, u8, u8);

#[wasm_bindgen]
pub struct Scene {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(x: usize, y: usize, nx: usize, ny: usize, ns: usize) -> Scene {
        let mut rng = rand::thread_rng();
        let capacity = x * y;
        let world = random_scene();
        let cam = camera::Camera::new(
            Vec3::new(3.0, 3.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            nx as f32 / ny as f32,
        );
        let mut pixels = Vec::with_capacity(capacity);
        let mut col = Vec3::new(0., 0., 0.);
        for i in 0..x {
            for j in 0..y {
                for _ in 0..ns {
                    let randi: f32 = rng.gen();
                    let randj: f32 = rng.gen();
                    let u = (i as f32 + randi) / nx as f32;
                    let v = ((ny - j) as f32 + randj) / ny as f32;
                    let r = cam.get_ray(u, v);
                    col += colour(r, &world, 0);
                }
                col /= ns as f32;
                col = Vec3 {
                    x: col.x.sqrt(),
                    y: col.y.sqrt(),
                    z: col.z.sqrt(),
                };

                let ir = (255.99 * col.x) as u8;
                let ig = (255.99 * col.y) as u8;
                let ib = (255.99 * col.z) as u8;

                pixels.push(Pixel(ir, ig, ib));
            }
        }

        Scene {
            width: x as u32,
            height: y as u32,
            pixels,
        }
    }
}

pub fn colour<T: hitable::Hitable>(r: Ray, world: &[T], depth: usize) -> Vec3 {
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

pub fn random_scene() -> Vec<Sphere> {
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
