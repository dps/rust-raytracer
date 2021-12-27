use jpeg_decoder::Decoder;
use palette::Srgb;
use rand::Rng;
use std::cmp::PartialEq;
use std::f64;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Point3D {
        let mut rng = rand::thread_rng();
        Point3D::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Point3D {
        loop {
            let p = Point3D::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x();
        let dy = self.y - other.y();
        let dz = self.z - other.z();
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.distance(&Point3D::new(0.0, 0.0, 0.0))
    }

    pub fn unit_vector(&self) -> Point3D {
        let length = self.length();
        Point3D::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn dot(&self, other: &Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point3D) -> Point3D {
        Point3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl Add for Point3D {
    type Output = Point3D;

    fn add(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Sub for Point3D {
    type Output = Point3D;

    fn sub(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Point3D {
        Point3D {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Point3D> for Point3D {
    type Output = Point3D;

    fn mul(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, other: f64) -> Point3D {
        Point3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<Point3D> for Point3D {
    type Output = Point3D;

    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, other: f64) -> Point3D {
        Point3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Point3D) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3D,
    pub direction: Point3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Point3D) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + self.direction * t
    }
}

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

#[test]
fn test_gen() {
    let p = Point3D {
        x: 0.1,
        y: 0.2,
        z: 0.3,
    };
    assert_eq!(p.x(), 0.1);
    assert_eq!(p.y(), 0.2);
    assert_eq!(p.z(), 0.3);

    let q = Point3D::new(0.2, 0.3, 0.4);
    assert_eq!(q.x(), 0.2);
    assert_eq!(q.y(), 0.3);
    assert_eq!(q.z(), 0.4);
}

#[test]
fn test_add() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);
    let r = p + q;
    assert_approx_eq!(r.x(), 0.3);
    assert_approx_eq!(r.y(), 0.5);
    assert_approx_eq!(r.z(), 0.7);
}

#[test]
fn test_sub() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);
    let r = p - q;
    assert_approx_eq!(r.x(), -0.1);
    assert_approx_eq!(r.y(), -0.1);
    assert_approx_eq!(r.z(), -0.1);
}

#[test]
fn test_neg() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = -p;
    assert_approx_eq!(q.x(), -0.1);
    assert_approx_eq!(q.y(), -0.2);
    assert_approx_eq!(q.z(), -0.3);
}

#[test]
fn test_mul() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);
    let r = p * q;
    assert_approx_eq!(r.x(), 0.02);
    assert_approx_eq!(r.y(), 0.06);
    assert_approx_eq!(r.z(), 0.12);
}

#[test]
fn test_div() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);
    let r = p / q;
    assert_approx_eq!(r.x(), 0.5);
    assert_approx_eq!(r.y(), 0.6666666666666666);
    assert_approx_eq!(r.z(), 0.3 / 0.4);
}

#[test]
fn test_dot() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);
    assert_approx_eq!(p.dot(&q), 0.2);
}

#[test]
fn test_length_squared() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    assert_approx_eq!(p.length_squared(), 0.14);
}

#[test]
fn test_random() {
    let p = Point3D::random(-1.0, 1.0);
    assert!(p.x() >= -1.0 && p.x() <= 1.0);
    assert!(p.y() >= -1.0 && p.y() <= 1.0);
    assert!(p.z() >= -1.0 && p.z() <= 1.0);
}

#[test]
fn test_near_zero() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    assert!(!p.near_zero());
    let p = Point3D::new(0.0, 0.0, 0.0);
    assert!(p.near_zero());
}

#[test]
fn test_ray() {
    let p = Point3D::new(0.1, 0.2, 0.3);
    let q = Point3D::new(0.2, 0.3, 0.4);

    let r = Ray::new(p, q);

    assert_approx_eq!(r.origin.x(), 0.1);
    assert_approx_eq!(r.origin.y(), 0.2);
    assert_approx_eq!(r.origin.z(), 0.3);
    assert_approx_eq!(r.direction.x(), 0.2);
    assert_approx_eq!(r.direction.y(), 0.3);
    assert_approx_eq!(r.direction.z(), 0.4);
}

#[test]
fn test_ray_at() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 2.0, 3.0);

    let r = Ray::new(p, q);
    let s = r.at(0.5);

    assert_approx_eq!(s.x(), 0.5);
    assert_approx_eq!(s.y(), 1.0);
    assert_approx_eq!(s.z(), 1.5);
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Glass(Glass),
    Texture(Texture),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Glass(g) => g.scatter(ray, hit_record),
            Material::Texture(t) => t.scatter(ray, hit_record),
        }
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
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)> {
        let mut scatter_direction = hit_record.normal + Point3D::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + Point3D::random_in_unit_sphere() * self.fuzz,
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, attenuation))
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)> {
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
            Some((scattered, attenuation))
        } else {
            let direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.point, direction);
            Some((scattered, attenuation))
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
        println!("{} loaded {:?}", texture_path, metadata);
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
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Srgb)> {
        let mut scatter_direction = hit_record.normal + Point3D::random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let target = hit_record.point + scatter_direction;
        let scattered = Ray::new(hit_record.point, target - hit_record.point);
        let attenuation = self.get_albedo(hit_record.u, hit_record.v);
        Some((scattered, attenuation))
    }
}

#[test]
fn test_texture() {
    let world = Material::Texture(Texture::new(
        Srgb::new(1.0, 1.0, 1.0),
        "data/earth.jpg",
        0.0,
    ));
}
