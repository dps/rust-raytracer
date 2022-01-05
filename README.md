# rust-raytracer

An implementation of a very simple raytracer based on [Ray Tracing in One Weekend
 by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust. I used this project to *learn* Rust from scratch - the code may not be perfectly idiomatic, or even good, but it does make pretty pictures.

Additional features beyond Shirley's course:
* Texture mapping (e.g. earth and moon textures below)
* Lighting
* Parallel rendering - will use all CPU cores for best performance
* Read scene data from JSON file
* Render a sky texture

## Example output
![Latest output](raytracer/output/cover.png)

https://user-images.githubusercontent.com/237355/147687883-4e9ca4fc-7c3b-4adb-85d7-6b08d1bc69f7.mp4

## Example usage
```
$ cargo build --release
   Compiling raytracer v0.1.0 (/Users/dps/proj/rust-raytracer/raytracer)
    Finished release [optimized] target(s) in 2.57s

$ ./target/release/raytracer data/test_scene.json out.png

Rendering out.png
Frame time: 2840ms

$ ./target/release/raytracer data/cover_scene.json cover.png

Rendering cover.png
Frame time: 27146ms
```

### Texture mapping
![cover_alt](https://user-images.githubusercontent.com/237355/147840674-38dd846f-1d4d-40a8-a573-e626a454f55a.png)

### Lighting
![lighting-recast-final](https://user-images.githubusercontent.com/237355/147840677-8e895fe5-1d25-428e-a847-6120af3ecfec.png)

### Parallel rendering - will use all CPU cores for best performance

#### Original
```
🚀 ./target/release/raytracer anim/frame
   Compiling raytracer v0.1.0 (/Users/dps/proj/rust-raytracer/raytracer)
    Finished release [optimized] target(s) in 2.21s

Rendering anim/frame_000.png
............................................................
Frame time: 21s
```
#### Using rayon
```
Rendering anim/frame_000.png
Frame time: 2573ms
```
### Render a sky texture
![sky_textures](https://user-images.githubusercontent.com/237355/147840693-355a75da-a473-4c44-b712-842129450306.gif)

### Read scene data from JSON file

#### Example
```
{
  "width": 800,
  "height": 600,
  "samples_per_pixel": 128,
  "max_depth": 50,
  "sky": {
    "texture":"data/beach.jpg"
  },
  "camera": {
    "look_from": { "x": -2.0, "y": 0.5, "z": 1.0 },
    "look_at": { "x": 0.0, "y": 0.0, "z": -1.0 },
    "vup": { "x": 0.0, "y": 1.0, "z": 0.0 },
    "vfov": 50.0,
    "aspect": 1.3333333333333333
  },
  "objects": [
    {
      "center": { "x": 0.0, "y": 0.0, "z": -1.0 },
      "radius": 0.5,
      "material": {
        "Texture": {
          "albedo": [
            1.0,
            1.0,
            1.0
          ],
          "pixels": "data/earth.jpg",
          "width": 2048,
          "height": 1024,
          "h_offset": 0.75
        }
      }
    }
  ]
}
```

### Make animation
```
🚀 ffmpeg -f image2 -framerate 15 -i anim/frame_%03d.png -loop -0 anim.gif
```

### Credits
Earth and moon textures from https://www.solarsystemscope.com/textures/

### Extreme lighting example
![147705264-c6f439df-f61b-4bcf-b5e6-c2c755b35b1c](https://user-images.githubusercontent.com/237355/147706272-7e35f213-914f-43dd-9b8b-4d3e7628cc19.png)

### Progressive max_depth animation
![max_depth_anim](https://user-images.githubusercontent.com/237355/148159509-aa492f3b-2805-45fe-94a6-3588fbf69bb2.gif)

