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

    #[bench]
    fn bench_threaded_unchecked_relaxation(bencher: &mut Bencher) {
        let w = 256;
        let h = 256;

        let mut x = Field::new(h, w, 0.0, 0.0);
        let b = Field::new(h, w, 0.0, 0.0);

        bencher.iter(|| Fluids::linear_solvers::threaded_relaxation_unchecked(&mut x, &b, 1.0, 0.01, 0.01, 100) );
    }

    #[bench]
    fn bench_unchecked_relaxation(bencher: &mut Bencher) {

        let w = 256;
        let h = 256;

        let mut x = Field::new(h, w, 0.0, 0.0);
        let b = Field::new(h, w, 0.0, 0.0);

        bencher.iter(|| Fluids::linear_solvers::relaxation_unchecked(&mut x, &b, 1.0, 0.01, 0.01, 100) );
    }

	#[bench]
    fn bench_relaxation(bencher: &mut Bencher) {

        let w = 256;
        let h = 256;

        let mut x = Field::new(h, w, 0.0, 0.0);
        let b = Field::new(h, w, 0.0, 0.0);

        bencher.iter(|| Fluids::linear_solvers::relaxation(&mut x, &b, 1.0, 0.01, 0.01, 100) );
    }
}
