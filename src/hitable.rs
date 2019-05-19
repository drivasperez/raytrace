use crate::matter::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Option<Material>,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, normal: Vec3, mat_ptr: Option<Material>) -> Self {
        HitRecord {
            t,
            p,
            normal,
            mat_ptr,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn mat_ptr(self) -> Option<Material>;
}

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    mat_ptr: Material,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, mat_ptr: Material) -> Self {
        Sphere {
            centre,
            radius,
            mat_ptr,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.centre;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.centre) / self.radius,
                    Some(self.mat_ptr.clone()),
                ));
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitRecord::new(
                    temp,
                    p,
                    (p - self.centre) / self.radius,
                    Some(self.mat_ptr.clone()),
                ));
            }
        };
        None
    }

    fn mat_ptr(self) -> Option<Material> {
        Some(self.mat_ptr)
    }
}

impl<T: Hitable> Hitable for &[T] {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        self.iter().for_each(|elem| {
            if let Some(temp) = elem.hit(r, t_min, closest_so_far) {
                closest_so_far = temp.t;
                rec = Some(temp);
            }
        });
        rec
    }

    fn mat_ptr(self) -> Option<Material> {
        None
    }
}
