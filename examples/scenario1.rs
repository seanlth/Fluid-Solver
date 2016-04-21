extern crate Fluids;

use Fluids::fluid_solver::*;
use Fluids::field::Field;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation;
use Fluids::advection;
use Fluids::integrators;
use Fluids::linear_solvers;


// marker particle
fn main() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 64, 128, 0.01, 1.0, 0.0)
                                   .use_markers()
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bicubic_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_fast_c);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..10000 {
        *solver.velocity_x.at_mut(32, 1) = 100.0;
        solver.solve();
        visualiser.draw_markers(&solver.particles, &solver.velocity_x, &solver.velocity_y);
    }
}
