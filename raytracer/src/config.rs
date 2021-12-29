use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::materials::Glass;
use crate::materials::Lambertian;
use crate::materials::Material;
use crate::materials::Metal;
use crate::point3d::Point3D;
use crate::sphere::Sphere;
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

fn _make_cover_world() -> Vec<Sphere> {
    let mut world = Vec::new();

    world.push(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Lambertian::new(Srgb::new(0.5, 0.5, 0.5))),
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3D::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if ((center - Point3D::new(4.0, 0.2, 0.0)).length()) < 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                // diffuse
                world.push(Sphere::new(
                    center,
                    0.2,
                    Material::Lambertian(Lambertian::new(Srgb::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ))),
                ));
            } else if choose_mat < 0.95 {
                // metal
                world.push(Sphere::new(
                    center,
                    0.2,
                    Material::Metal(Metal::new(
                        Srgb::new(
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                            0.5 * (1.0 + rng.gen::<f32>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    )),
                ));
            } else {
                // glass
                world.push(Sphere::new(center, 0.2, Material::Glass(Glass::new(1.5))));
            }
        }
    }

    world.push(Sphere::new(
        Point3D::new(0.0, 1.0, 0.0),
        1.0,
        Material::Glass(Glass::new(1.5)),
    ));
    world.push(Sphere::new(
        Point3D::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Lambertian::new(Srgb::new(
            0.4 as f32, 0.2 as f32, 0.1 as f32,
        ))),
    ));
    world.push(Sphere::new(
        Point3D::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Metal::new(
            Srgb::new(0.7 as f32, 0.6 as f32, 0.5 as f32),
            0.0,
        )),
    ));
    world
}

#[test]
fn test_cover_scene_to_json() {
    let config = Config {
        width: 800,
        height: 600,
        samples_per_pixel: 64,
        max_depth: 50,
        sky: true,
        camera: Camera::new(
            Point3D::new(13.0, 2.0, 3.0),
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
            20.0,
            (800.0 / 600.0) as f64,
        ),
        objects: _make_cover_world(),
    };
    let serialized = serde_json::to_string_pretty(&config).unwrap();
    fs::write("/tmp/cover_scene.json", serialized).unwrap();
}

#[test]
fn test_from_file() {
    let json = fs::read("data/test_scene.json").expect("Unable to read file");
    let scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    assert_eq!(scene.width, 800);
    assert_eq!(scene.height, 600);
}
