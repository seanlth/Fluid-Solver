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
    let mut solver:FluidSolver = FluidSolver::new(1.0, 128, 64, 0.01, 1.0, 9.8)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::euler)
                                   .linear_solver(linear_solvers::relaxation_fast_c);
    let visualiser = Visualiser::new(solver.rows, solver.columns);

    for i in 0..201 {
        // *solver.velocity_x.at_mut(32, 64) = 150.0;
        solver.solve();
        visualiser.draw_pressure(&solver.pressure);
        // if i % 50 == 0 {
        //     let s = format!("images/pressure{}.png", i);
        //     visualiser.to_image(&*s);
        // }
    }
    solver.print_pressure();
}
