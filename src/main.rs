mod ray;
mod vec3;
use ray::Ray;
use std::fs::File;
use std::io::prelude::*;
use vec3::Vec3;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::create("cool2.ppm")?;
    let nx = 200;
    let ny = 100;
    writeln!(file, "P3\n{} {}\n255", nx, ny)?;

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
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}

fn colour(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = Vec3::unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Vec3::new(n.x + 1., n.y + 1., n.z + 1.);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * unit_direction.y + 1.0;
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(centre: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - centre;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    match discriminant < 0. {
        true => -1.0,
        false => (-b - discriminant.sqrt()) / (2.0 * a),
    }
}
