use jpeg_decoder::Decoder;
use palette::Srgb;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fs::File;
use std::io::BufReader;

use crate::camera::Camera;
use crate::materials::Glass;
use crate::materials::Lambertian;
use crate::materials::Material;
use crate::materials::Metal;
use crate::point3d::Point3D;
use crate::sphere::Sphere;

#[cfg(test)]
use std::fs;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sky {
    // If provided, the sky will be rendered using the equirectangular
    // projected texture loaded from an image file at this path. Else,
    // a light blue colored sky will be used.
    #[serde_as(as = "TextureOptionPixelsAsPath")]
    pub texture: Option<(Vec<u8>, usize, usize, String)>,
}

impl Sky {
    pub fn new_default_sky() -> Sky {
        Sky { texture: None }
    }
}

fn load_texture_image(path: &str) -> (Vec<u8>, usize, usize, String) {
    let file = File::open(path).expect(path);
    let mut decoder = Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    (
        pixels,
        metadata.width as usize,
        metadata.height as usize,
        path.to_string(),
    )
}

serde_with::serde_conv!(
    TextureOptionPixelsAsPath,
    Option<(Vec<u8>, usize, usize, String)>,
    |texture: &Option<(Vec<u8>, usize, usize, String)>| {
        match texture {
            Some(tuple) => tuple.3.clone(),
            None => "".to_string(),
        }
    },
    |value: &str| -> Result<_, std::convert::Infallible> {
        match value {
            "" => Ok(None),
            _ => Ok(Some(load_texture_image(value))),
        }
    }
);

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: u32,
    pub max_depth: usize,
    pub sky: Option<Sky>,
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
        sky: Some(Sky::new_default_sky()),
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
    assert_eq!("{\"width\":100,\"height\":100,\"samples_per_pixel\":1,\"max_depth\":1,\"sky\":{\"texture\":\"\"},\"camera\":{\"look_from\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"look_at\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"vup\":{\"x\":0.0,\"y\":1.0,\"z\":0.0},\"vfov\":90.0,\"aspect\":1.0},\"objects\":[{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"radius\":0.5,\"material\":{\"Lambertian\":{\"albedo\":[0.8,0.3,0.3]}}}]}", serialized);
}

#[test]
fn test_sky_perms_to_from_json() {
    let config = Config {
        width: 100,
        height: 100,
        samples_per_pixel: 1,
        max_depth: 1,
        sky: None,
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
    assert_eq!("{\"width\":100,\"height\":100,\"samples_per_pixel\":1,\"max_depth\":1,\"sky\":null,\"camera\":{\"look_from\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"look_at\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"vup\":{\"x\":0.0,\"y\":1.0,\"z\":0.0},\"vfov\":90.0,\"aspect\":1.0},\"objects\":[{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"radius\":0.5,\"material\":{\"Lambertian\":{\"albedo\":[0.8,0.3,0.3]}}}]}", serialized);
    let _ = serde_json::from_str::<Config>(&serialized).expect("Unable to parse json");

    // This scene contains a sky texture at data/earth,jpg
    let scene_json = "{\"width\":100,\"height\":100,\"samples_per_pixel\":1,\"max_depth\":1,\"sky\":{\"texture\":\"data/earth.jpg\"},\"camera\":{\"look_from\":{\"x\":0.0,\"y\":0.0,\"z\":0.0},\"look_at\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"vup\":{\"x\":0.0,\"y\":1.0,\"z\":0.0},\"vfov\":90.0,\"aspect\":1.0},\"objects\":[{\"center\":{\"x\":0.0,\"y\":0.0,\"z\":-1.0},\"radius\":0.5,\"material\":{\"Lambertian\":{\"albedo\":[0.8,0.3,0.3]}}}]}";
    let scene = serde_json::from_str::<Config>(&scene_json).expect("Unable to parse json");

    assert_eq!(
        match scene.sky {
            Some(sky) => {
                match sky.texture {
                    Some(tuple) => (tuple.1, tuple.2, tuple.3),
                    _ => (0, 0, "".to_string()),
                }
            }
            _ => (0, 0, "".to_string()),
        },
        (2048, 1024, "data/earth.jpg".to_string())
    )
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
        sky: Some(Sky::new_default_sky()),
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
