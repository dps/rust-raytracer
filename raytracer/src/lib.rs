use std::f64;

pub mod camera;
pub mod materials;
pub mod point3d;
pub mod ray;

use materials::Material;
use point3d::Point3D;
use ray::Ray;

#[cfg(test)]
use materials::Lambertian;
#[cfg(test)]
use palette::Srgb;

pub struct HitRecord<'material> {
    pub t: f64,
    pub point: Point3D,
    pub normal: Point3D,
    pub front_face: bool,
    pub material: &'material Material,
    pub u: f64,
    pub v: f64,
}

impl<'material> HitRecord<'material> {
    pub fn new(
        t: f64,
        point: Point3D,
        normal: Point3D,
        front_face: bool,
        material: &'material Material,
        u: f64,
        v: f64,
    ) -> HitRecord<'material> {
        HitRecord {
            t,
            point,
            normal,
            front_face,
            material,
            u,
            v,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3D,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

fn u_v_from_sphere_hit_point(hit_point_on_sphere: Point3D) -> (f64, f64) {
    let n = hit_point_on_sphere.unit_vector();
    let x = n.x();
    let y = n.y();
    let z = n.z();
    let u = (x.atan2(z) / (2.0 * std::f64::consts::PI)) + 0.5;
    let v = y * 0.5 + 0.5;
    (u, v)
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let root_a = ((-half_b) - sqrtd) / a;
            let root_b = ((-half_b) + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = ray.direction.dot(&normal) < 0.0;

                    let (u, v) = u_v_from_sphere_hit_point(p - self.center);

                    return Some(HitRecord {
                        t: *root,
                        point: p,
                        normal: if front_face { normal } else { -normal },
                        front_face,
                        material: &self.material,
                        u,
                        v,
                    });
                }
            }
        }
        None
    }
}

#[test]
fn test_sphere_hit() {
    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(
        center,
        1.0,
        Material::Lambertian(Lambertian::new(Srgb::new(
            0.5 as f32, 0.5 as f32, 0.5 as f32,
        ))),
    );
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert_eq!(hit.unwrap().t, 4.0);
}
