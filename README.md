# Staggered-Grid Fluid Solver in Rust

Code repository for my Final Year Project in Trinity College, Dublin.

## Building
* Requires Rust nightly, tested with Rust 1.8.0-nightly
* Requires clang++ or g++
* ```make build``` to build the project
* ```make run``` to run the solver

## Crates  
* ```glium``` for realtime visualisation
* ```lodepng-rust``` for generating density images
* ```scoped-threadpool``` for scoped threading
* ```rust-opencl``` for OpenCL bindings

## Project Overview
* Chorin projection
* Staggered grid
* Linear, Cubic, Catmull-Rom and Hermite interpolators
* Euler, Bogacki-Shampine and Runge-Kutta 4 integrators
* Density, Pressure and Marker-particle visualisers
