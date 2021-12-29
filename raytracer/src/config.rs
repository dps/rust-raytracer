use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::sphere::Sphere;
use crate::materials::Material;
use crate::materials::Lambertian;
use crate::point3d::Point3D;

#[cfg(test)]
use palette::Srgb;
#[cfg(test)]
use std::fs;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: usize,
    pub sky: bool,
    pub camera: Camera,
    pub objects: Vec<Sphere>,
}

#[test]
fn test_to_json() {
    let config = Config {
        width: 100,
        height: 100,
        samples_per_pixel: 1,
        max_depth: 1,
        sky: true,
        camera: Camera::new(
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 0.0, -1.0),
            Point3D::new(0.0, 1.0, 0.0),
            90.0,
            1.0,
        ),
        objects: vec![Sphere::new(
            Point3D::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Lambertian::new(Srgb::new(
                0.8 as f32, 0.3 as f32, 0.3 as f32,
            ))),
        )],
    };
    let serialized = serde_json::to_string(&config).unwrap();
    assert_eq!("{\"width\":100,\"height\":100,\"samples_per_pixel\":1,\"max_depth\":1,\"sky\":true,\"camera\":{\"look_from\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"look_at\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"vup\":{\"x\":0.0,\"y\":1.0,\"z\":0.0},\"vfov\":90.0,\"aspect\":1.0},\"objects\":[{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"radius\":0.5,\"material\":{\"Lambertian\":{\"albedo\":[0.8,0.3,0.3]}}}]}", serialized);
}

#[test]
fn test_from_file() {
    let json = fs::read("data/scene.json").expect("Unable to read file");
    let scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    assert_eq!(scene.width, 800);
    assert_eq!(scene.height, 600);
}