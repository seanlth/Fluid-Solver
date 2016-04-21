extern crate Fluids;

use Fluids::fluid_solver::*;
use Fluids::field::Field;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation;
use Fluids::advection;
use Fluids::integrators;
use Fluids::linear_solvers;

// pressure
fn main() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 128, 128, 0.01, 1.0, 0.0)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_opencl);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..10000 {
        *solver.velocity_x.at_mut(31, 49) = 800.0;
        *solver.density.at_mut(31, 49) = 2.0;
        *solver.velocity_y.at_mut(49, 256 - 32) = 800.0;
        *solver.density.at_mut(49, 256 - 32) = 2.0;

        solver.solve();
        visualiser.draw_density_rgb(&solver.density);
    }
}
