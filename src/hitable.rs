use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
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

impl Hitable for Vec<Box<Hitable>> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for elem in self {
            if elem.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
