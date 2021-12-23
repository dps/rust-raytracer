use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
use std::f64;

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

impl Div for Point3D {
    type Output = Point3D;

    fn div(self, other: Point3D) -> Point3D {
        Point3D {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
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

#[test]
fn test_gen() {
    let p = Point3D{x:0.1, y:0.2, z:0.3};
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
    assert_approx_eq!(r.z(), 0.3/0.4);
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