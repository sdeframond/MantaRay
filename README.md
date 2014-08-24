MantaRay
========

This is an experimental ray tracer/path tracer written in Rust.

## Examples

see [here](./examples/)

## Usage

Edit the scene in `mains.rs`, then `Cargo run`. The output will be generated in `result.png`.

## Features

- Global illumination (comes with soft shadows and caustics).
- Different shapes: sphere and infinite plane.
- Diffuse material.
- Reflection (mirror).
- Refraction (glass, water etc).
- Emittive material.

## Todo

- Use basic command line arguments (output file, input files, bounce depth, bounce factor...).
- Implement triangle meshes.
- Implement some file import from 3DS or Blender.
- Implement bidirectionnal path tracing.
- Use with the GPU.
