# Staggered-Grid Fluid Solver in Rust

Code repository for my Final Year Project in Trinity College, Dublin.

## Building source

1. Dependencies
* Rust nightly ( tested with Rust 1.8.0-nightly )
* clang or gcc
* OpenCL runtime

2. Building
* ```make build``` to build the project
* ```make run``` to run the solver

## Crates
* ```glium``` for realtime visualisation
* ```lodepng-rust``` for generating density images
* ```scoped-threadpool``` for scoped threading
* ```rust-opencl``` for OpenCL bindings

## Overview
* Staggered grid method
* Chorin projection
* Implements ppwind advection and semi-lagrangian advection
* Implements linear, cubic, Catmull-Rom and Hermite interpolators
* Euler, Bogacki-Shampine and Runge-Kutta-4 integrators
* Implements simple relaxation method for pressure solver
* Density, Pressure and Marker-particle visualisers
