# Staggered-Grid Fluid Solver in Rust

![screenshot](https://github.com/seanlth/Fluid-Solver/blob/master/image.png)

## Building source

1. Dependencies
    * Rust nightly ( tested with Rust 1.8.0-nightly )
    * clang or gcc
    * OpenCL runtime

2. Building
    * ```make build``` to build the project

3. Examples
    * ```make example``` to build the examples

4. Benchmarks
    * ```make bench``` to run benchmarks

## Crates
* ```glium``` for realtime visualisation
* ```lodepng-rust``` for generating density images
* ```scoped-threadpool``` for scoped threading
* ```rust-opencl``` for OpenCL bindings

## Project Overview

This Eulerian solver uses the Chorin projection method on a staggered grid.

Chorin projection decouples velocity and pressure in the momentum equation allowing them to be calculated separately. The advection step can be performed using the upwind scheme or the semi-lagrangian scheme. Multiple integrators and interpolators have been implemented with varying degrees of accuracy. The pressure solve step uses the simple jacobi relaxation linear solver. It has been implemented in Rust, C and OpenCL with varying degrees of optimisation.

A staggered grid is used to prevent checkerboarding when calculating the pressure gradient.

Density, pressure and marker-particle visualisation methods have been implemented using the glium and lodepng crates.

## Project layout


### `fluid_solver.rs`
* Ties together all the algorithms required
* Contains the various fields associated with a fluid and the functions that manage the solver

### `advection.rs`
* Upwind advection implementation
* Semi-Lagrangian advection implementation

### `interpolation.rs`
* Linear interpolation
* Cubic interpolation
* Catmull-Rom interpolation
* Hermite interpolation

### `integrators.rs`
* Euler integrator
* Bogacki-Shampine integrator
* Runge-Kutta 4 integrator

### `linear_solvers.rs`
* Rust, C, OpenCL implementations of Jacobi relaxation

### `visualiser.rs`
* Density visualisation
* Inverse density visualisation
* Density visualisation with jet colourmap
* Pressure visualisation with jet colourmap
* Marker-particle visualisation
