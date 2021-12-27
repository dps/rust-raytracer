# rust-raytracer

An implementation of https://raytracing.github.io/books/RayTracingInOneWeekend.html in Rust

## Latest output
![Latest output](raytracer/output/cover.png)
![Rotating texture mapped earth gif](raytracer/output/mvanim.gif)


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

### Make animation
```
🚀 ffmpeg -f image2 -framerate 15 -i anim/frame_%03d.png -loop -0 anim.gif
```

### Credits
Earth and moon textures from https://www.solarsystemscope.com/textures/
