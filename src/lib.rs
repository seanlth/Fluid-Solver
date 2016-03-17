#[macro_use]
extern crate glium;
extern crate opencl;
extern crate crossbeam;
extern crate scoped_threadpool;

pub mod fluid_solver;
pub mod linear_solvers;
pub mod integrators;
pub mod visualiser;
pub mod field;
pub mod interpolation;
pub mod advection;
