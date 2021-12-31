use serde::{Deserialize, Serialize};

use crate::materials::Material;
use crate::point3d::Point3D;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::ray::Ray;

#[cfg(test)]
use crate::materials::Glass;
#[cfg(test)]
use crate::materials::Lambertian;
#[cfg(test)]
use crate::materials::Texture;
#[cfg(test)]
use palette::Srgb;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sphere {
    pub center: Point3D,
    pub radius: f64,
    pub material: Material,
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
    let sphere = Sphere::new(center, 1.0, Material::Glass(Glass::new(1.5)));
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert_eq!(hit.unwrap().t, 4.0);
}

#[test]
fn test_to_json() {
    let sphere = Sphere::new(
        Point3D::new(0.0, 0.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Srgb::new(
            0.5 as f32, 0.5 as f32, 0.5 as f32,
        ))),
    );
    let serialized = serde_json::to_string(&sphere).unwrap();
    assert_eq!(
        "{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"radius\":1.0,\"material\":{\"Lambertian\":{\"albedo\":[0.5,0.5,0.5]}}}",
        serialized,
    );
    let s = serde_json::from_str::<Sphere>(&serialized).unwrap();
    assert_eq!(sphere.center, s.center);
    assert_eq!(sphere.radius, s.radius);

    let textured_sphere = Sphere::new(
        Point3D::new(0.0, 0.0, 0.0),
        1.0,
        Material::Texture(Texture::new(
            Srgb::new(0.5 as f32, 0.5 as f32, 0.5 as f32),
            "data/earth.jpg",
            0.0,
        )),
    );

    let tserialized = serde_json::to_string(&textured_sphere).unwrap();
    assert_eq!(
        "{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"radius\":1.0,\"material\":{\"Texture\":{\"albedo\":[0.5,0.5,0.5],\"pixels\":\"/tmp/texture.jpg\",\"width\":2048,\"height\":1024,\"h_offset\":0.0}}}",
        tserialized,
    );

    let tex = Texture::new(
        Srgb::new(0.5 as f32, 0.5 as f32, 0.5 as f32),
        "data/earth.jpg",
        0.0,
    );
    let tloadable = "{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"radius\":1.0,\"material\":{\"Texture\":{\"albedo\":[0.5,0.5,0.5],\"pixels\":\"data/earth.jpg\",\"width\":2048,\"height\":1024,\"h_offset\":0.0}}}";
    let loaded = serde_json::from_str::<Sphere>(&tloadable).unwrap();
    match loaded.material {
        Material::Texture(ref t) => {
            assert_eq!(t.pixels, tex.pixels);
        }
        _ => panic!("Wrong material type"),
    }
}
