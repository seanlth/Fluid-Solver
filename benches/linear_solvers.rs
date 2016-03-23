#![feature(test)]

extern crate test;
extern crate Fluids;

use Fluids::linear_solvers::*;
use Fluids::field::Field;


#[cfg(test)]
mod tests {
	use Fluids;
	use Fluids::linear_solvers::*;
    use Fluids::field::Field;
    use test::Bencher;

    // #[bench]
    // fn bench_add_opencl(bencher: &mut Bencher) {
    //     let vec1 = vec![0.0; 10000000];
    //     let vec2 = vec![0.0; 10000000];
    //     let mut vec3 = vec![0.0; 10000000];
    //
    //     bencher.iter(|| Fluids::linear_solvers::add_opencl_test(&vec1, &vec2, &mut vec3) );
    // }
    //
    // #[bench]
    // fn bench_add(bencher: &mut Bencher) {
    //     let vec1 = vec![0.0; 10000000];
    //     let vec2 = vec![0.0; 10000000];
    //     let mut vec3 = vec![0.0; 10000000];
    //
    //     bencher.iter(|| Fluids::linear_solvers::add_test(&vec1, &vec2, &mut vec3) );
    // }

    // #[bench]
    // fn bench_relaxation_opencl(bencher: &mut Bencher) {
    //     let w = 128;
    //     let h = 128;
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation_opencl(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    // }

    #[bench]
    fn bench_relaxation_c(bencher: &mut Bencher) {
        let w = 128;
        let h = 128;

        let mut x = Field::new(h, w, 0.0, 0.0);
        let b = Field::new(h, w, 0.0, 0.0);

        bencher.iter(|| Fluids::linear_solvers::relaxation_c(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    }

    #[bench]
    fn bench_relaxation_fast_c(bencher: &mut Bencher) {
        let w = 128;
        let h = 128;

        let mut x = Field::new(h, w, 0.0, 0.0);
        let b = Field::new(h, w, 0.0, 0.0);

        bencher.iter(|| Fluids::linear_solvers::relaxation_fast_c(&mut x, &b, 1.0, 0.01, 0.01, 600) );
    }
    //
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
    //     let w = 256;
    //     let h = 256;
    //
    //     let mut x = Field::new(h, w, 0.0, 0.0);
    //     let b = Field::new(h, w, 0.0, 0.0);
    //
    //     bencher.iter(|| Fluids::linear_solvers::relaxation_unchecked(&mut x, &b, 1.0, 0.01, 0.01, 100) );
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
