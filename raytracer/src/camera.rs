use serde::{Deserialize, Serialize};

use crate::point3d::Point3D;
use crate::ray::Ray;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(from = "CameraParams")]
pub struct Camera {
    #[serde(skip_serializing)]
    pub origin: Point3D, // Note, don't serialize any of the computed fields.
    #[serde(skip_serializing)]
    pub lower_left_corner: Point3D,
    #[serde(skip_serializing)]
    pub focal_length: f64,
    #[serde(skip_serializing)]
    pub horizontal: Point3D,
    #[serde(skip_serializing)]
    pub vertical: Point3D,
    look_from: Point3D,
    look_at: Point3D,
    vup: Point3D,
    vfov: f64, // vertical field-of-view in degrees
    aspect: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraParams {
    pub look_from: Point3D,
    pub look_at: Point3D,
    pub vup: Point3D,
    pub vfov: f64, // vertical field-of-view in degrees
    pub aspect: f64,
}

impl From<CameraParams> for Camera {
    fn from(p: CameraParams) -> Self {
        Camera::new(p.look_from, p.look_at, p.vup, p.vfov, p.aspect)
    }
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
            look_from,
            look_at,
            vup,
            vfov,
            aspect,
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
fn test_to_json() {
    let camera = Camera::new(
        Point3D::new(-4.0, 4.0, 1.0),
        Point3D::new(0.0, 0.0, -1.0),
        Point3D::new(0.0, 1.0, 0.0),
        160.0,
        (800 / 600) as f64,
    );
    let serialized = serde_json::to_string(&camera).unwrap();
    assert_eq!("{\"look_from\":{\"x\":-4.0,\"y\":4.0,\"z\":1.0},\"look_at\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"vup\":{\"x\":0.0,\"y\":1.0,\"z\":0.0},\"vfov\":160.0,\"aspect\":1.0}", serialized);
    let c = serde_json::from_str::<Camera>(&serialized).unwrap();
    assert_eq!(camera.origin, c.origin);
    assert_eq!(camera.lower_left_corner, c.lower_left_corner);
    assert_eq!(camera.focal_length, c.focal_length);
    assert_eq!(camera.horizontal, c.horizontal);
    assert_eq!(camera.vertical, c.vertical);
}
