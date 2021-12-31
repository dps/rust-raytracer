use image::png::PNGEncoder;
use image::ColorType;
use palette::Pixel;
use palette::Srgb;
use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::time::Instant;

use crate::config::Config;
use crate::materials::Material;
use crate::materials::Scatterable;
use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;

#[cfg(test)]
use std::fs;

#[cfg(test)]
use crate::point3d::Point3D;

#[cfg(test)]
use crate::camera::Camera;
#[cfg(test)]
use crate::config::Sky;
#[cfg(test)]
use crate::materials::Lambertian;
#[cfg(test)]
use crate::materials::Light;

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

fn hit_world<'material>(
    world: &'material Vec<Sphere>,
    r: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

fn clamp(value: f32) -> f32 {
    if value < 0.0 {
        0.0
    } else if value > 1.0 {
        1.0
    } else {
        value
    }
}

fn ray_color(
    ray: &Ray,
    scene: &Config,
    lights: &Vec<Sphere>,
    max_depth: usize,
    depth: usize,
) -> Srgb {
    let mut rng = rand::thread_rng();

    if depth <= 0 {
        return Srgb::new(0.0, 0.0, 0.0);
    }
    let hit = hit_world(&scene.objects, ray, 0.001, std::f64::MAX);
    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(ray, &hit_record);
            match scattered {
                Some((scattered_ray, albedo)) => {
                    let mut light_red = 0.0;
                    let mut light_green = 0.0;
                    let mut light_blue = 0.0;
                    let mut prob = 0.1;
                    match hit_record.material {
                        Material::Glass(_) => {
                            prob = 0.05;
                        }
                        _ => {}
                    }
                    if lights.len() > 0
                        && rng.gen::<f64>() > (1.0 - lights.len() as f64 * prob)
                        && depth > (max_depth - 2)
                    {
                        for light in lights {
                            let light_ray =
                                Ray::new(hit_record.point, light.center - hit_record.point);
                            let target_color = ray_color(&light_ray, scene, lights, 2, 1);
                            light_red += albedo.red * target_color.red;
                            light_green += albedo.green * target_color.green;
                            light_blue += albedo.blue * target_color.blue;
                        }
                        light_red /= lights.len() as f32;
                        light_green /= lights.len() as f32;
                        light_blue /= lights.len() as f32;
                    }
                    match scattered_ray {
                        Some(sr) => {
                            let target_color = ray_color(&sr, scene, lights, max_depth, depth - 1);
                            return Srgb::new(
                                clamp(light_red + albedo.red * target_color.red),
                                clamp(light_green + albedo.green * target_color.green),
                                clamp(light_blue + albedo.blue * target_color.blue),
                            );
                        }
                        None => albedo,
                    }
                }
                None => {
                    // don't bother bouncing absorbed rays towards lights
                    // (they would be absorbed in the opposite direction).
                    return Srgb::new(0.0, 0.0, 0.0);
                }
            }
        }
        None => {
            let t: f32 = clamp(0.5 * (ray.direction.unit_vector().y() as f32 + 1.0));
            let u: f32 = clamp(0.5 * (ray.direction.unit_vector().x() as f32 + 1.0));
            match &scene.sky {
                None => {
                    return Srgb::new(0.0, 0.0, 0.0);
                }
                Some(sky) => match &sky.texture {
                    None => {
                        return Srgb::new(
                            (1.0 - t) * 1.0 + t * 0.5,
                            (1.0 - t) * 1.0 + t * 0.7,
                            (1.0 - t) * 1.0 + t * 1.0,
                        );
                    }
                    Some((pixels, width, height, _)) => {
                        let x = (u * (*width - 1) as f32) as usize;
                        let y = ((1.0 - t) * (*height - 1) as f32) as usize;
                        let pixel_red = &pixels[(y * *width + x) * 3];
                        let pixel_green = &pixels[(y * *width + x) * 3 + 1];
                        let pixel_blue = &pixels[(y * *width + x) * 3 + 2];
                        return Srgb::new(
                            0.7 * *pixel_red as f32 / 255.0,
                            0.7 * *pixel_green as f32 / 255.0,
                            0.7 * *pixel_blue as f32 / 255.0,
                        );
                    }
                },
            }
        }
    }
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    let scene = Config {
        width: 80,
        height: 60,
        samples_per_pixel: 1,
        max_depth: 2,
        sky: Some(Sky::new_default_sky()),
        camera: Camera::new(
            Point3D::new(0.0, 0.0, -3.0),
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
            20.0,
            1.333,
        ),
        objects: Vec::new(),
    };
    let l = Vec::new();
    assert_eq!(ray_color(&r, &scene, &l, 2, 2), Srgb::new(0.75, 0.85, 1.0));
}

fn render_line(pixels: &mut [u8], scene: &Config, lights: &Vec<Sphere>, y: usize) {
    let mut rng = rand::thread_rng();

    let bounds = (scene.width, scene.height);

    for x in 0..bounds.0 {
        let mut pixel_colors: Vec<f32> = vec![0.0; 3];
        for _s in 0..scene.samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (bounds.0 as f64 - 1.0);
            let v = (bounds.1 as f64 - (y as f64 + rng.gen::<f64>())) / (bounds.1 as f64 - 1.0);
            let r = scene.camera.get_ray(u, v);
            let c = ray_color(&r, scene, lights, scene.max_depth, scene.max_depth);
            pixel_colors[0] += c.red;
            pixel_colors[1] += c.green;
            pixel_colors[2] += c.blue;
        }
        let scale = 1.0 / scene.samples_per_pixel as f32;
        let color = Srgb::new(
            (scale * pixel_colors[0]).sqrt(),
            (scale * pixel_colors[1]).sqrt(),
            (scale * pixel_colors[2]).sqrt(),
        );
        let pixel: [u8; 3] = color.into_format().into_raw();
        pixels[x * 3] = pixel[0];
        pixels[x * 3 + 1] = pixel[1];
        pixels[x * 3 + 2] = pixel[2];
    }
}

fn find_lights(world: &Vec<Sphere>) -> Vec<Sphere> {
    world
        .iter()
        .filter(|s| match s.material {
            Material::Light(_) => true,
            _ => false,
        })
        .cloned()
        .collect()
}

#[test]
fn test_find_lights() {
    let world = vec![
        Sphere::new(
            Point3D::new(0.0, 0.0, -1.0),
            0.5,
            Material::Light(Light::new()),
        ),
        Sphere::new(
            Point3D::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertian(Lambertian::new(Srgb::new(
                0.5 as f32, 0.5 as f32, 0.5 as f32,
            ))),
        ),
    ];
    assert_eq!(find_lights(&world).len(), 1);
}

pub fn render(filename: &str, scene: Config) {
    let image_width = scene.width;
    let image_height = scene.height;

    let mut pixels = vec![0; image_width * image_height * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(image_width * 3).enumerate().collect();

    let lights = find_lights(&scene.objects);

    let start = Instant::now();
    bands.into_par_iter().for_each(|(i, band)| {
        render_line(band, &scene, &lights, i);
    });
    println!("Frame time: {}ms", start.elapsed().as_millis());

    write_image(filename, &pixels, (image_width, image_height)).expect("error writing image");
}

#[test]
fn test_render_full_test_scene() {
    let json = fs::read("data/test_scene.json").expect("Unable to read file");
    let mut scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    scene.width = 80;
    scene.height = 60;
    render("/tmp/test_scene.png", scene);
}

#[test]
fn test_render_full_cover_scene() {
    let json = fs::read("data/cover_scene.json").expect("Unable to read file");
    let mut scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse json");
    scene.width = 40;
    scene.height = 30;
    render("/tmp/cover_scene.png", scene);
}
