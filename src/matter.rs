use crate::hitable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f32),
    Dielectric(f32),
}

impl Material {
    pub fn scatter(self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        match self {
            Material::Lambertian(albedo) => scatter_lambertian(albedo, r_in, rec),
            Material::Metal(albedo, fuzz) => scatter_metal(albedo, fuzz, r_in, rec),
            Material::Dielectric(ri) => scatter_dielectric(ri, r_in, rec),
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> (Vec3, bool) {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        (ni_over_nt * (uv - n * dt) - n * discriminant.sqrt(), true)
    } else {
        (Vec3::default(), false)
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

fn scatter_lambertian(albedo: Vec3, _r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    (albedo, Ray::new(rec.p, target - rec.p), true)
}

fn scatter_metal(albedo: Vec3, fuzz: f32, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
    let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
    let scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());

    (
        albedo,
        scattered,
        Vec3::dot(scattered.direction(), rec.normal) > 0.,
    )
}

fn scatter_dielectric(ref_idx: f32, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
    let mut rng = rand::thread_rng();
    let attenuation = Vec3::new(1.0, 1.0, 0.0);
    let reflected = reflect(r_in.direction(), rec.normal);
    let direction_dot_normal = Vec3::dot(r_in.direction(), rec.normal);

    let (outward_normal, ni_over_nt, cosine) = if direction_dot_normal > 0. {
        (
            -rec.normal,
            ref_idx,
            ref_idx * direction_dot_normal / r_in.direction().length(),
        )
    } else {
        (
            rec.normal,
            1.0 / ref_idx,
            -direction_dot_normal / r_in.direction().length(),
        )
    };

    let (refracted, did_refract) = refract(r_in.direction(), outward_normal, ni_over_nt);

    let reflect_prob = if did_refract {
        schlick(cosine, ref_idx)
    } else {
        1.0
    };

    let roll: f32 = rng.gen();

    let scattered = if roll < reflect_prob {
        Ray::new(rec.p, reflected)
    } else {
        Ray::new(rec.p, refracted)
    };

    (attenuation, scattered, true)
}
