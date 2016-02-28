#![feature(test)]

extern crate test;
extern crate Fluids;

use Fluids::linear_solvers::*;


#[cfg(test)]
mod tests {
    use super::*;
	use Fluids;
	use Fluids::linear_solvers::*;
    use test::Bencher;


    #[bench]
    fn bench_unchecked_relaxation(b: &mut Bencher) {

		let mut x: Vec<f64> = vec![0.0; 10201];
		let mut d: Vec<f64> = vec![0.0; 10201];


        b.iter(|| Fluids::linear_solvers::relaxation_unchecked(&mut x, &d, 101, 101, 1.0, 0.01, 1.0, 100) );
    }

	#[bench]
    fn bench_relaxation(b: &mut Bencher) {

		let mut x: Vec<f64> = vec![0.0; 10201];
		let mut d: Vec<f64> = vec![0.0; 10201];


        b.iter(|| Fluids::linear_solvers::relaxation(&mut x, &d, 101, 101, 1.0, 0.01, 1.0, 100) );
    }
}
