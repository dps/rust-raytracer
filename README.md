# rust-raytracer

An implementation of a very simple raytracer based on [Ray Tracing in One Weekend
 by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust. I used this project to *learn* Rust from scratch - the code may not be perfectly idiomatic, or even good, but it does make pretty pictures.
 
Additional features beyond Shirley's course:
* Texture mapping (e.g. earth and moon textures below)
* Lighting
* Parallel rendering - will use all CPU cores for best performance
* Read scene data from JSON file

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

### Perf profiling
```
🚀 ./target/release/raytracer anim/frame
   Compiling raytracer v0.1.0 (/Users/dps/proj/rust-raytracer/raytracer)
    Finished release [optimized] target(s) in 2.21s

Rendering anim/frame_000.png
............................................................Frame time: 21s

Rendering anim/frame_001.png
............................................................Frame time: 21s

Rendering anim/frame_002.png
............................................................Frame time: 20s
```
Using `crossbeam` to distribute across 8 threads
```
Rendering anim/frame_000.png
Frame time: 5s

Rendering anim/frame_001.png
Frame time: 5s

Rendering anim/frame_002.png
Frame time: 5s
```
Uneven chunk timing
```
Rendering anim/frame_003.png
Chunk time: 573ms
Chunk time: 776ms
Chunk time: 1728ms
Chunk time: 4180ms
Chunk time: 5215ms
Chunk time: 5428ms
Chunk time: 5632ms
Chunk time: 5705ms
Frame time: 5735ms
```
Using rayon
```
Rendering anim/frame_000.png
Frame time: 2573ms

Rendering anim/frame_001.png
Frame time: 2775ms

Rendering anim/frame_002.png
Frame time: 3049ms

Rendering anim/frame_003.png
Frame time: 3299ms
```

### Make animation
```
🚀 ffmpeg -f image2 -framerate 15 -i anim/frame_%03d.png -loop -0 anim.gif
```

### Credits
Earth and moon textures from https://www.solarsystemscope.com/textures/

### Extreme lighting example
![147705264-c6f439df-f61b-4bcf-b5e6-c2c755b35b1c](https://user-images.githubusercontent.com/237355/147706272-7e35f213-914f-43dd-9b8b-4d3e7628cc19.png)

