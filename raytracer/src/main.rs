use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File; 
use std::env;

fn write_image(filename: &str, pixels: &[u8], bounds:(usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::RGB(8))?;
    Ok(())
}

fn render(pixels: &mut[u8], bounds: (usize, usize)) {
    assert!(pixels.len() == bounds.0 * bounds.1 * 3);

    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            let r = (x as f32 / (bounds.0 as f32 - 1.0)) as f32;
            let g = (y as f32 / (bounds.1 as f32 - 1.0)) as f32;
            let b = 0.25;
            let i = y * bounds.0 + x;
            pixels[i * 3] = (255.99 * r) as u8;
            pixels[i * 3 + 1] = (255.99 * g) as u8;
            pixels[i * 3 + 2] = (255.99 * b) as u8;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_width = 800;
    let image_height = 600;

    let mut pixels = vec![0; image_width * image_height * 3];

    println!("raytracer {}x{}", image_width, image_height);
    if args.len() != 2 {
        println!("Usage: {} <output_file>", args[0]);
        return;
    }

    render(&mut pixels, (image_width, image_height));

    write_image(&args[1], &pixels, (image_width, image_height)).expect("error writing image");

}
