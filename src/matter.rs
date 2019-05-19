use crate::hitable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
        match self {
            Material::Lambertian(albedo) => scatter_lambertian(*albedo, r_in, rec),
            Material::Metal(albedo) => scatter_metal(*albedo, r_in, rec),
        }
    }
}

fn scatter_lambertian(albedo: Vec3, _r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    (albedo, Ray::new(rec.p, target - rec.p), true)
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(v, n) * n
}

fn scatter_metal(albedo: Vec3, r_in: &Ray, rec: &HitRecord) -> (Vec3, Ray, bool) {
    let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
    let scattered = Ray::new(rec.p, reflected);

    (
        albedo,
        scattered,
        Vec3::dot(scattered.direction(), rec.normal) > 0.,
    )
}
