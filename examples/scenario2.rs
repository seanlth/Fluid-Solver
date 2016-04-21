extern crate Fluids;

use Fluids::fluid_solver::*;
use Fluids::field::Field;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation;
use Fluids::advection;
use Fluids::integrators;
use Fluids::linear_solvers;

// rgb density
fn main() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 64, 256, 0.01, 1.0, 0.0)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_fast_c);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for _ in 0..256 {
        *solver.velocity_x.at_mut(31, 49) = 100.0;
        *solver.density.at_mut(31, 49) = 2.0;

        solver.solve();
        visualiser.draw_density(&solver.density);
    }
}
