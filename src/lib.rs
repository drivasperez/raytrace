pub mod camera;
pub mod hitable;
pub mod matter;
pub mod ray;
pub mod vec3;
use cfg_if::cfg_if;
use hitable::{Hitable, Sphere};
use js_sys::Math::random;
use matter::Material;
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
    iterations: u32,
    pixels: Vec<(u8, u8, u8)>,
    camera: camera::Camera,
    world: Vec<Sphere>,
}

#[wasm_bindgen]
impl Scene {
    pub fn move_camera(&mut self, camera: camera::Camera) {
        self.camera = camera;
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn pixels(&self) -> *const (u8, u8, u8) {
        self.pixels.as_ptr()
    }

    pub fn draw(&mut self) {
        let nx = self.width;
        let ny = self.height;
        let ns = self.iterations;
        self.pixels.clear();

        let mut col = Vec3::new(0., 0., 0.);
        for i in 0..nx {
            for j in 0..ny {
                for _ in 0..ns {
                    let randi: f32 = random() as f32;
                    let randj: f32 = random() as f32;
                    let u = (i as f32 + randi) / nx as f32;
                    let v = ((ny - j) as f32 + randj) / ny as f32;
                    let r = self.camera.get_ray(u, v);
                    col += colour(r, &self.world, 0);
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

                self.pixels.push((ir, ig, ib));
            }
        }
    }

    pub fn new(nx: usize, ny: usize, ns: usize, sphere_pos: Vec3) -> Scene {
        let capacity = nx * ny;
        let world = random_scene(sphere_pos);

        let camera = camera::Camera::new(
            Vec3::new(3.0, 3.0, 2.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            60.0,
            nx as f32 / ny as f32,
        );
        let pixels = Vec::with_capacity(capacity);

        Scene {
            width: nx as u32,
            height: ny as u32,
            iterations: ns as u32,
            pixels,
            camera,
            world,
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
    let mut p = 2.0 * (Vec3::new(random() as f32, random() as f32, random() as f32))
        - Vec3::new(1., 1., 1.);

    while p.squared_length() >= 1.0 {
        p = 2.0 * (Vec3::new(random() as f32, random() as f32, random() as f32))
            - Vec3::new(1., 1., 1.);
    }
    p
}

pub fn random_scene(sphere_pos: Vec3) -> Vec<Sphere> {
    let mut hit_list = Vec::new();
    hit_list.push(Sphere::new(
        Vec3::new(0.0, -1000., 0.0),
        1000.0,
        Material::Lambertian(Vec3::new(0.5, 0.5, 0.5)),
    ));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat: f32 = random() as f32;
    //         let (x, y) = (a as f32, b as f32);
    //         let shiftx: f32 = random() as f32;
    //         let shifty: f32 = random() as f32;
    //         let center = Vec3::new(x + 0.9 * shiftx, 0.2, y + 0.9 * shifty);
    //         if (center - Vec3::new(4.0, 0.0, 2.0)).length() > 0.9 {
    //             if choose_mat < 0.8 {
    //                 hit_list.push(Sphere::new(
    //                     center,
    //                     0.2,
    //                     Material::Lambertian(Vec3::new(
    //                         random() as f32,
    //                         random() as f32,
    //                         random() as f32,
    //                     )),
    //                 ))
    //             } else if choose_mat < 0.95 {
    //                 hit_list.push(Sphere::new(
    //                     center,
    //                     0.2,
    //                     Material::Metal(
    //                         Vec3::new(
    //                             0.5 * random() as f32,
    //                             0.5 * random() as f32,
    //                             0.5 * random() as f32,
    //                         ),
    //                         0.5 * random() as f32,
    //                     ),
    //                 ))
    //             } else {
    //                 hit_list.push(Sphere::new(center, 0.2, Material::Dielectric(1.5)))
    //             }
    //         };
    //     }
    // }

    hit_list.push(Sphere::new(sphere_pos, 1.0, Material::Dielectric(1.5)));
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
