extern crate Fluids;
extern crate lodepng;

use Fluids::fluid_solver::*;
use Fluids::field::Field;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation;
use Fluids::advection;
use Fluids::integrators;
use Fluids::linear_solvers;


fn run() {
    let mut solver:FluidSolver = FluidSolver::new(1.0, 256, 256, 0.01, 1.0, 0.0)
                                   .advection(advection::semi_lagrangian)
                                   .interpolation(interpolation::bilinear_interpolate)
                                   .integration(integrators::runge_kutta_4)
                                   .linear_solver(linear_solvers::relaxation_opencl);
    let visualiser = Visualiser::new();

    for _ in 0..1000 {
        solver.add_source(128, 4, 20000.0, 0.0, 10.0);
        // solver.add_source(511, 253, 0.0, -1000.0, 100.0);
        // solver.add_source(511, 254, 0.0, -1000.0, 100.0);
        // solver.add_source(511, 255, 0.0, -1000.0, 100.0);
        // solver.add_source(511, 256, 0.0, -1000.0, 100.0);
        // solver.add_source(511, 257, 0.0, -1000.0, 100.0);
        // solver.add_source(511, 258, 0.0, -1000.0, 100.0);

        solver.solve();
        visualiser.draw_density_inverse(&solver.density);
        //visualiser.draw_density_rgb(&solver.density);
        //solver.print_density();
        //visualiser.draw_markers(&solver.particles, solver.columns, solver.rows);
        //visualiser.draw_pressure(&solver.pressure);
    }
    //visualiser.draw_density_image(&solver.density, &*format!("images/density.png"));
}

fn main() {
    run();
}
