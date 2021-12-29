use std::env;
use std::fs;

use raytracer::config::Config;
use raytracer::raytracer::render;

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
