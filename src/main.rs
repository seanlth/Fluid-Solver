extern crate Fluids;
extern crate lodepng;

use Fluids::fluid_solver::*;
use Fluids::field::Field;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation;
use Fluids::advection;
use Fluids::integrators;
use Fluids::linear_solvers;


// marker particle
fn scenario1() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 64, 128, 0.01, 1.0, 0.0)
                                   .use_markers()
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bicubic_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_fast_c);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..10000 {
        //solver.add_source(32, 1, 100.0, 0.0, 10.0);
        *solver.velocity_x.at_mut(32, 1) = 100.0;
        solver.solve();
        visualiser.draw_markers(&solver.particles, solver.columns, solver.rows);
    }
}

// rgb density
fn scenario2() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 64, 256, 0.005, 1.0, 0.0)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_opencl);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..10000 {
        *solver.velocity_x.at_mut(31, 49) = 400.0;
        *solver.density.at_mut(31, 49) = 2.0;
        *solver.velocity_x.at_mut(31, 45) = 400.0;

        solver.solve();
        visualiser.draw_density_rgb(&solver.density);
    }
}

// pressure
fn scenario3() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 64, 256, 0.01, 1.0, 0.0)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_opencl);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..10000 {
        *solver.velocity_x.at_mut(31, 49) = 100.0;
        solver.solve();
        visualiser.draw_pressure(&solver.pressure);
    }
}

// fn run() {
//     let mut solver:FluidSolver = FluidSolver::new(1.0, 128, 128, 0.01, 1.0, 0.0)
//                                    .advection(advection::semi_lagrangian)
//                                    .interpolation(interpolation::bicubic_interpolate)
//                                    .integration(integrators::euler)
//                                    .linear_solver(linear_solvers::relaxation_fast_c);
//     let visualiser = Visualiser::new();
//
//     for _ in 0..1000 {
//         solver.add_source(64, 1, 1000.0, 0.0, 10.0);
//         //solver.add_source(65, 1, 500.0, 0.0, 10.0);
//
//         //solver.add_source(128, 4, 1000.0, 0.0, 10.0);
//         // solver.add_source(511, 253, 0.0, -1000.0, 100.0);
//         // solver.add_source(511, 254, 0.0, -1000.0, 100.0);
//         // solver.add_source(511, 255, 0.0, -1000.0, 100.0);
//         // solver.add_source(511, 256, 0.0, -1000.0, 100.0);
//         // solver.add_source(511, 257, 0.0, -1000.0, 100.0);
//         // solver.add_source(511, 258, 0.0, -1000.0, 100.0);
//
//         solver.solve();
//         //visualiser.draw_density_inverse(&solver.density);
//         //visualiser.draw_density_rgb(&solver.density);
//         //solver.print_density();
//         visualiser.draw_markers(&solver.particles, solver.columns, solver.rows);
//         //visualiser.draw_pressure(&solver.pressure);
//     }
//     //visualiser.draw_density_image(&solver.density, &*format!("images/density.png"));
// }

fn main() {
    scenario3();
}
