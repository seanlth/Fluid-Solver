#![feature(test)]

extern crate test;
extern crate Fluids;




#[cfg(test)]
mod tests {
	use Fluids;
    use Fluids::fluid_solver::*;
    use Fluids::field::Field;
    use Fluids::interpolation;
    use Fluids::advection;
    use Fluids::integrators;
    use Fluids::linear_solvers;
    use test::Bencher;

    const w: usize = 512;
    const h: usize = 512;

    // #[bench]
    // fn bench_linear_solver_opencl(bencher: &mut Bencher) {
    //
    //     let mut solver:FluidSolver = FluidSolver::new(1.0, w, h, 0.01, 1.0, 0.0)
    //                                    .advection(advection::semi_lagrangian)
    //                                    .interpolation(interpolation::bilinear_interpolate)
    //                                    .integration(integrators::bogacki_shampine)
    //                                    .linear_solver(linear_solvers::relaxation_opencl);
    //
    //     bencher.iter(|| solver.solve() );
    // }

    // #[bench]
    // fn bench_opencl(bencher: &mut Bencher) {
    //
    //
    //     let mut solver:FluidSolver = FluidSolver::new(1.0, w, h, 0.01, 1.0, 0.0)
    //                                    .advection(advection::semi_lagrangian)
    //                                    .interpolation(interpolation::bilinear_interpolate)
    //                                    .integration(integrators::bogacki_shampine)
    //                                    .linear_solver(linear_solvers::relaxation_opencl);
    //
    //     bencher.iter(|| solver.solve() );
    // }

    // #[bench]
    // fn bench_fast_c(bencher: &mut Bencher) {
    //
    //     let mut solver:FluidSolver = FluidSolver::new(1.0, w, h, 0.01, 1.0, 0.0)
    //                                    .advection(advection::semi_lagrangian)
    //                                    .interpolation(interpolation::bilinear_interpolate)
    //                                    .integration(integrators::bogacki_shampine)
    //                                    .linear_solver(linear_solvers::relaxation_fast_c);
    //
    //     bencher.iter(|| solver.solve() );
    // }

    // #[bench]
    // fn bench_relaxation_c(bencher: &mut Bencher) {
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation_c(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    // }

    // #[bench]
    // fn bench_relaxation_fast_c(bencher: &mut Bencher) {
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation_fast_c(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    // }

    //
    // #[bench]
    // fn bench_threaded_unchecked_relaxation(bencher: &mut Bencher) {
    //     let w = 256;
    //     let h = 256;
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::threaded_relaxation_unchecked(&mut x, &b, 1.0, 0.01, 0.01, 100) );
    // }
    //
    // #[bench]
    // fn bench_unchecked_relaxation(bencher: &mut Bencher) {
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation_unchecked(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    // }
    //
	// #[bench]
    // fn bench_relaxation(bencher: &mut Bencher) {
    //
    //     let w = 256;
    //     let h = 256;
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation(&mut x, &b, 1.0, 0.01, 0.01, 100) );
    // }
}
