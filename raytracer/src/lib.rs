use rand::Rng;
use std::cmp::PartialEq;
use std::f64;
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
        origin: Point3D,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = Point3D::new(viewport_width, 0.0, 0.0);
        let vertical = Point3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Point3D::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            focal_length,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}

#[test]
fn test_camera() {
    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        2.0,
        (800 / 600) as f64 * 2.0,
        1.0,
    );
    assert_eq!(camera.origin.x(), 0.0);
    assert_eq!(camera.origin.y(), 0.0);
    assert_eq!(camera.origin.z(), 0.0);

    assert_eq!(camera.lower_left_corner.x(), -1.0);
    assert_eq!(camera.lower_left_corner.y(), -1.0);
    assert_eq!(camera.lower_left_corner.z(), -1.0);
}

#[test]
fn test_camera_get_ray() {
    let camera = Camera::new(
        Point3D::new(0.0, 0.0, 0.0),
        2.0,
        (800 / 600) as f64 * 2.0,
        1.0,
    );
    let ray = camera.get_ray(0.5, 0.5);
    assert_eq!(ray.origin.x(), 0.0);
    assert_eq!(ray.origin.y(), 0.0);
    assert_eq!(ray.origin.z(), 0.0);

    assert_eq!(ray.direction.x(), 0.0);
    assert_eq!(ray.direction.y(), 0.0);
    assert_eq!(ray.direction.z(), -1.0);
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

pub struct HitRecord {
    pub t: f64,
    pub point: Point3D,
    pub normal: Point3D,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: Point3D, normal: Point3D, front_face: bool) -> HitRecord {
        HitRecord {
            t,
            point,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3D,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3D, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let normal = (p - self.center) / self.radius;
                let front_face = ray.direction.dot(&normal) < 0.0;

                return Some(HitRecord {
                    t: temp,
                    point: p,
                    normal: if front_face { normal } else { -normal },
                    front_face,
                });
            }
        }
        None
    }
}

#[test]
fn test_sphere_hit() {
    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0);
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, 1.0));
    let hit = sphere.hit(&ray, 0.0, f64::INFINITY);
    assert_eq!(hit.unwrap().t, 4.0);
}
