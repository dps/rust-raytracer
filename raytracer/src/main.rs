use image::png::PNGEncoder;
use image::ColorType;
use palette::Pixel;
use palette::Srgb;
use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::time::Instant;

pub mod camera;
pub mod config;
pub mod materials;
pub mod point3d;
pub mod ray;
pub mod sphere;

use camera::Camera;
use config::Config;
use materials::Glass;
use materials::Lambertian;
use materials::Light;
use materials::Material;
use materials::Metal;
use materials::Scatterable;
use materials::Texture;
use point3d::Point3D;
use ray::HitRecord;
use ray::Hittable;
use ray::Ray;
use sphere::Sphere;

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

fn ray_color(ray: &Ray, world: &Vec<Sphere>, depth: usize, sky: bool) -> Srgb {
    if depth <= 0 {
        return Srgb::new(0.0, 0.0, 0.0);
    }
    let hit = hit_world(world, ray, 0.001, std::f64::MAX);
    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(ray, &hit_record);
            match scattered {
                Some((scattered_ray, albedo)) => match scattered_ray {
                    Some(sr) => {
                        let target_color = ray_color(&sr, world, depth - 1, sky);
                        return Srgb::new(
                            albedo.red * target_color.red,
                            albedo.green * target_color.green,
                            albedo.blue * target_color.blue,
                        );
                    }
                    None => albedo,
                },
                None => {
                    return Srgb::new(0.0, 0.0, 0.0);
                }
            }
        }
        None => {
            let t: f32 = 0.5 * (ray.direction.unit_vector().y() as f32 + 1.0);
            if sky {
                return Srgb::new(
                    (1.0 - t) * 1.0 + t * 0.5,
                    (1.0 - t) * 1.0 + t * 0.7,
                    (1.0 - t) * 1.0 + t * 1.0,
                );
            } else {
                return Srgb::new(0.0, 0.0, 0.0);
            }
        }
    }
}

#[test]
fn test_ray_color() {
    let p = Point3D::new(0.0, 0.0, 0.0);
    let q = Point3D::new(1.0, 0.0, 0.0);
    let r = Ray::new(p, q);
    let w = Vec::new();
    assert_eq!(ray_color(&r, &w, 2, true), Srgb::new(0.75, 0.85, 1.0));
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

fn render_line(
    pixels: &mut [u8],
    bounds: (usize, usize),
    world: &Vec<Sphere>,
    camera: &Camera,
    samples_per_pixel: u32,
    max_depth: usize,
    sky: bool,
    y: usize,
) {
    let mut rng = rand::thread_rng();

    for x in 0..bounds.0 {
        let mut pixel_colors: Vec<f32> = vec![0.0; 3];
        for _s in 0..samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (bounds.0 as f64 - 1.0);
            let v = (bounds.1 as f64 - (y as f64 + rng.gen::<f64>())) / (bounds.1 as f64 - 1.0);
            let r = camera.get_ray(u, v);
            let c = ray_color(&r, &world, max_depth, sky);
            pixel_colors[0] += c.red;
            pixel_colors[1] += c.green;
            pixel_colors[2] += c.blue;
        }
        let scale = 1.0 / samples_per_pixel as f32;
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

fn render(filename: &str, scene: Config) {
    let image_width = scene.width;
    let image_height = scene.height;

    let mut pixels = vec![0; image_width * image_height * 3];
    let bounds = (image_width, image_height);
    let bands: Vec<(usize, &mut [u8])> = pixels.chunks_mut(bounds.0 * 3).enumerate().collect();

    let start = Instant::now();

    bands.into_par_iter().for_each(|(i, band)| {
        render_line(
            band,
            bounds,
            &(scene.objects),
            &(scene.camera),
            scene.samples_per_pixel,
            scene.max_depth,
            scene.sky,
            i,
        );
    });
    println!("Frame time: {}ms", start.elapsed().as_millis());

    write_image(filename, &pixels, (image_width, image_height)).expect("error writing image");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <config_file> <output_file>", args[0]);
        return;
    }

    let json = fs::read(&args[1]).expect("Unable to read config file.");
    let scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse config json");

    let filename = &args[2]; //format!("{}_{:0>3}.png", args[2], i);
    println!("\nRendering {}", filename);
    render(&filename, scene);
}
