use jpeg_decoder::Decoder;
use palette::Srgb;
use rand::Rng;
use std::f64;
use std::fs::File;
use std::io::BufReader;

pub mod point3d;
pub mod ray;

use point3d::Point3D;
use ray::Ray;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub origin: Point3D,
    pub lower_left_corner: Point3D,
    pub focal_length: f64,
    pub horizontal: Point3D,
    pub vertical: Point3D,
}

impl Camera {
    pub fn new(
        look_from: Point3D,
        look_at: Point3D,
        vup: Point3D,
        vfov: f64, // vertical field-of-view in degrees
        aspect: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = origin - (u * half_width) - (v * half_height) - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Camera {
            origin,
            lower_left_corner,
            focal_length: (look_from - look_at).length(),
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}

#[test]
fn test_camera() {
    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        Point3D::new(0.0, 0.0, -1.0),
        Point3D::new(0.0, 1.0, 0.0),
        90.0,
        (800.0 / 600.0) as f64,
    );
    assert_eq!(camera.origin.x(), 0.0);
    assert_eq!(camera.origin.y(), 0.0);
    assert_eq!(camera.origin.z(), 0.0);

    assert_approx_eq!(camera.lower_left_corner.x(), -(1.0 + (1.0 / 3.0)));
    assert_approx_eq!(camera.lower_left_corner.y(), -1.0);
    assert_approx_eq!(camera.lower_left_corner.z(), -1.0);
}

#[test]
fn test_camera_get_ray() {
    let camera = Camera::new(
        Point3D::new(-4.0, 4.0, 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        Point3D::new(0.0, 1.0, 0.0),
        160.0,
        (800 / 600) as f64,
    );
    let ray = camera.get_ray(0.5, 0.5);
    assert_eq!(ray.origin.x(), -4.0);
    assert_eq!(ray.origin.y(), 4.0);
    assert_eq!(ray.origin.z(), 1.0);

    assert_approx_eq!(ray.direction.x(), (2.0 / 3.0));
    assert_approx_eq!(ray.direction.y(), -(2.0 / 3.0));
    assert_approx_eq!(ray.direction.z(), -(1.0 / 3.0));
}

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

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Glass(Glass),
    Texture(Texture),
    Light(Light),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Glass(g) => g.scatter(ray, hit_record),
            Material::Texture(t) => t.scatter(ray, hit_record),
            Material::Light(l) => l.scatter(ray, hit_record),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {}

impl Light {
    pub fn new() -> Light {
        Light {}
    }
}

impl Scatterable for Light {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        Some((None, Srgb::new(1.0, 1.0, 1.0)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Srgb,
}

impl Lambertian {
    pub fn new(albedo: Srgb) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut scatter_direction = hit_record.normal + Point3D::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;
        Some((Some(scattered), attenuation))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Srgb,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Srgb, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

fn reflect(v: &Point3D, n: &Point3D) -> Point3D {
    *v - *n * (2.0 * v.dot(n))
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + Point3D::random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((Some(scattered), attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Glass {
    pub index_of_refraction: f64,
}

impl Glass {
    pub fn new(index_of_refraction: f64) -> Glass {
        Glass {
            index_of_refraction,
        }
    }
}

fn refract(uv: &Point3D, n: &Point3D, etai_over_etat: f64) -> Point3D {
    let cos_theta = ((-*uv).dot(n)).min(1.0);
    let r_out_perp = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[test]
fn test_refract() {
    let uv = Point3D::new(1.0, 1.0, 0.0);
    let n = Point3D::new(-1.0, 0.0, 0.0);
    let etai_over_etat = 1.0;
    let expected = Point3D::new(0.0, 1.0, 0.0);
    let actual = refract(&uv, &n, etai_over_etat);
    assert_eq!(actual, expected);
}

#[test]
fn test_reflectance() {
    let cosine = 0.0;
    let ref_idx = 1.5;
    let expected = 1.0;
    let actual = reflectance(cosine, ref_idx);
    assert_eq!(actual, expected);
}

impl Scatterable for Glass {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut rng = rand::thread_rng();
        let attenuation = Srgb::new(1.0 as f32, 1.0 as f32, 1.0 as f32);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            let reflected = reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.point, reflected);
            Some((Some(scattered), attenuation))
        } else {
            let direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.point, direction);
            Some((Some(scattered), attenuation))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub albedo: Srgb,
    pixels: Vec<u8>,
    width: u64,
    height: u64,
    h_offset: f64,
}

impl Texture {
    pub fn new(albedo: Srgb, texture_path: &str, rot: f64) -> Texture {
        let file = File::open(texture_path).expect("failed to open texture file");
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();
        Texture {
            albedo,
            pixels,
            width: metadata.width as u64,
            height: metadata.height as u64,
            h_offset: rot,
        }
    }

    pub fn get_albedo(&self, u: f64, v: f64) -> Srgb {
        let mut rot = u + self.h_offset;
        if rot > 1.0 {
            rot = rot - 1.0;
        }
        let uu = rot * (self.width) as f64;
        let vv = (1.0 - v) * (self.height - 1) as f64;
        let base_pixel =
            (3 * ((vv.floor() as u64) * self.width as u64 + (uu.floor() as u64))) as usize;
        let pixel_r = self.pixels[base_pixel];
        let pixel_g = self.pixels[base_pixel + 1];
        let pixel_b = self.pixels[base_pixel + 2];
        Srgb::new(
            pixel_r as f32 / 255.0,
            pixel_g as f32 / 255.0,
            pixel_b as f32 / 255.0,
        )
    }
}

impl Scatterable for Texture {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Srgb)> {
        let mut scatter_direction = hit_record.normal + Point3D::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.get_albedo(hit_record.u, hit_record.v);
        Some((Some(scattered), attenuation))
    }
}

#[test]
fn test_texture() {
    let _world = Material::Texture(Texture::new(
        Srgb::new(1.0, 1.0, 1.0),
        "data/earth.jpg",
        0.0,
    ));
}
