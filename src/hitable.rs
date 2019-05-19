use crate::matter::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Box<Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t: 0.0,
            normal: Vec3::default(),
            p: Vec3::default(),
            mat_ptr: Box::new(crate::matter::Lambertian::new(0.0, 0.0, 0.0)),
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    mat_ptr: Box<Material>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, mat_ptr: Box<Material>) -> Self {
        Sphere {
            centre,
            radius,
            mat_ptr,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.centre;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                return true;
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.centre) / self.radius;
                return true;
            }
        };
        false
    }
}

impl<T: Hitable> Hitable for &[T] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        self.iter().for_each(|elem| {
            if elem.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        });
        hit_anything
    }
}
