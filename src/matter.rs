use crate::hitable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Copy, Clone, Default)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(rec.p, target - rec.p);
        *attenuation = self.albedo;
        true
    }
}

impl Lambertian {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Lambertian {
            albedo: Vec3::new(x, y, z),
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}

#[derive(Copy, Clone, Default)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Metal {
            albedo: Vec3::new(x, y, z),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), rec.normal) > 0.
    }
}
