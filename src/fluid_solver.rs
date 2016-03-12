extern crate lodepng;

use std::mem::swap;
use std::io::prelude::*;
use std::fs::File;

use field::Field;
use linear_solvers;
use integrators;
use advection;
use interpolation;

pub fn matrix(row: i32, column: i32) -> Vec<Vec<f64>> {
	let mut t: Vec<Vec<f64>> = Vec::new();

	for _ in 0..row {
		let mut t2 = Vec::new();
		for _ in 0..column {
			t2.push(0.0);
		}
		t.push(t2);
	}
	t
}

// pub struct FluidSolver {
// 	pub pressure: Vec<Vec<f64>>,
// 	pub u: Vec<Vec<f64>>,
// 	pub v: Vec<Vec<f64>>,
// 	pub
// }


#[derive(Clone)]
pub struct Variable {
	pub values: Vec<Vec<f64>>,
	pub temp: Vec<Vec<f64>>,
	offset_x: f64,
	offset_y: f64,
}

impl Variable {
	pub fn new(values:  Vec<Vec<f64>>, offset_x: f64, offset_y: f64) -> Variable {
		Variable {
			values: values.clone(),
			temp: values,
			offset_x: offset_x,
			offset_y: offset_y
		}
	}

	pub fn new_zeroed(rows: i32, columns: i32, offset_x: f64, offset_y: f64) -> Variable {
		Variable {
			values: matrix(rows, columns),
			temp: matrix(rows, columns),
			offset_x: offset_x,
			offset_y: offset_y
		}
	}

	pub fn interpolate_1d(&self, a: f64, b: f64, w: f64) -> f64 {
		a * w + b * (1.0 - w)
	}

	pub fn clamp(&self, v: f64, a: f64, b: f64) -> f64 {
		if v < a { a }
		else if v > b { b }
		else { v }
	}


	// interpolate across the field
	pub fn interpolate_2d(&self, mut x: f64, mut y: f64) -> f64 {

		//println!("x = {}", x);

		// adjust coordinates to relative to variable space
		x = x - self.offset_x;
		y = y - self.offset_y;

		//println!("x = {}", x);

		if x == 0.5 && y == 4.5 {
		// println!("{}", x);
		// println!("{}", y);
		}

		// clamp coordinates within field
		x = self.clamp(x, 0.0, self.values[0].len() as f64 - 1.0);
		y = self.clamp(y, 0.0, self.values.len() as f64 - 1.0);

		//println!("{}", x);

		// points to consider in interpolation
		// p1 - - - p2
		// |        |
		// |   p    |
		// |        |
		// p3 - - - p4
	    let (p1_x, p1_y) = ( self.clamp((x+1.0).floor(), 0.0, self.values[0].len() as f64 - 1.0), self.clamp(y.floor(), 0.0, self.values.len() as f64 - 1.0) );
	    let (p2_x, p2_y) = ( self.clamp(x.floor(), 0.0, self.values[0].len() as f64 - 1.0), self.clamp(y.floor(), 0.0, self.values.len() as f64 - 1.0) );
	    let (p3_x, p3_y) = ( self.clamp((x+1.0).floor(), 0.0, self.values[0].len() as f64 - 1.0), self.clamp((y+1.0).floor(), 0.0, self.values.len() as f64 - 1.0) );
	    let (p4_x, p4_y) = ( self.clamp(x.floor(), 0.0, self.values[0].len() as f64 - 1.0), self.clamp((y+1.0).floor(), 0.0, self.values.len() as f64 - 1.0) );

		//if x == 0.5 && y == 4.5 {
			// println!("x = {}, y = {}", x, y);
			// println!("p1 = {}, {}", p1_x, p1_y);
			// println!("p2 = {}, {}", p2_x, p2_y);
			// println!("p3 = {}, {}", p3_x, p3_y);
			// println!("p4 = {}, {}", p4_x, p4_y);
		//}

 		// weight from 0 to 1 in x and y axis
	    let alpha = y - p2_y;
	    let beta = x - p2_x;
		//if r == 4.0 && c == 0.0 {

		// println!("{}", alpha);
		// println!("{}", beta);
		//
		// println!("x = {}, y = {}", x, y);
		// println!("{}, {}", p1_x, p1_y);
		// println!("{}, {}", p2_x, p2_y);
		// println!("{}, {}", p3_x, p3_y);
		// println!("{}, {}", p4_x, p4_y);

		//}

		let p1 = self.values[p1_y as usize][p1_x as usize];
		let p2 = self.values[p2_y as usize][p2_x as usize];
		let p3 = self.values[p3_y as usize][p3_x as usize];
		let p4 = self.values[p4_y as usize][p4_x as usize];

		// println!("p1 = {} @ {}, {}", p1, p1_x, p1_y);
		// println!("p2 = {} @ {}, {}", p2, p2_x, p2_y);
		// println!("p3 = {} @ {}, {}", p3, p3_x, p3_y);
		// println!("p4 = {} @ {}, {}", p4, p4_x, p4_y);


		// interpolate in x-axis
		let l1 = self.interpolate_1d(p1, p2, beta);
		let l2 = self.interpolate_1d(p3, p4, beta);



		// interpolate in y-axis
		self.interpolate_1d(l2, l1, alpha)
	}

	// forward euler back trace
	// see diagram __
	pub fn integrate(&self, x: f64, y: f64, u: &Variable, v: &Variable, dt: f64) -> (f64, f64) {

		// x velocity at point
		let u_p = u.interpolate_2d(x, y);
		let v_p = v.interpolate_2d(x, y);

		//let u_p = interpolation::bilinear_interpolate(x - u.offset_x, y - u.offset_y, &u.values);
		//let v_p = interpolation::bilinear_interpolate(x - v.offset_x, y - v.offset_y, &v.values);

		let new_x = x as f64 - u_p * dt;
		let new_y = y as f64 - v_p * dt;
		//let new_y = y as f64;

		//println!("{}, {} ", u_p, v_p);
		// println!("{}, {} ", x, y);

		(new_x, new_y)
	}

	// semi-lagrangian advection
	// see diagram __
	pub fn advect(&mut self, u: &Variable, v: &Variable, dt: f64) {
		let w = self.values[0].len();
		let h = self.values.len();

		for r in 0..h {
			for c in 0..w {
				// integrate from location of variable within grid
				let (old_x, old_y) = self.integrate(c as f64 + self.offset_x, r as f64 + self.offset_y, &u, &v, dt);

				//let (old_x, old_y) = (c as f64 + self.offset_x, r as f64 + self.offset_y);

				//println!("{}, {} -> {}, {}", old_x, old_y, c as f64 + self.offset_x, r as f64 + self.offset_y);

				self.temp[r][c] = self.interpolate_2d(old_x, old_y);

				//self.temp[r][c] = interpolation::bilinear_interpolate(old_x - self.offset_x, old_y - self.offset_y, &self.values);
			}
		}
	}
}


pub struct FluidSolver {
	pub velocity_x: Field,
	pub velocity_y: Field,
	pub density: Field,
	pub pressure: Field,
	pub divergence: Field,
	pub particles: Vec<(f64, f64)>,
	pub rows: usize,
	pub columns: usize,
	dt: f64,
	dx: f64,
	fluid_density: f64,
}

impl FluidSolver {
	pub fn new(fluid_density: f64, rows: usize, columns: usize, dt: f64, dx: f64) -> FluidSolver {

		let mut particles = Vec::new();

		for i in 0..rows*2 {
			for j in 0..columns*2 {
				particles.push( (i as f64 * dx / 2.0, j as f64*dx / 2.0 ) );
			}
		}

 		FluidSolver {
 			velocity_x: Field::new(rows, columns+1, 0.0, 0.5),
 			velocity_y: Field::new(rows+1, columns, 0.5, 0.0),
			density: Field::new(rows, columns, 0.5, 0.5),
			particles: particles,
			pressure: Field::new(rows, columns, 0.5, 0.5),
			divergence: Field::new(rows, columns, 0.5, 0.5),
			fluid_density: fluid_density,
 			rows: rows,
 			columns: columns,
			dt: dt,
			dx: dx
 		}
 	}



	// LP = D
	// see diagram
	pub fn pressure_solve(&mut self) {
		linear_solvers::relaxation( &mut self.pressure, &self.divergence, self.columns, self.rows, self.fluid_density, self.dt, self.dx, 200 );
	}

	pub fn solve(&mut self) {
		//self.apply_gravity();
		self.project();
		self.advect();
	}

	pub fn apply_gravity(&mut self) {
		for r in 0..self.rows+1 {
			for c in 0..self.columns {
				*self.velocity_y.at_mut(r, c) -= 10.0;
			}
		}
	}

	pub fn advect_particles(&mut self) {

		for p in &mut self.particles {
			let (x, y) = *p;
			let u = interpolation::bicubic_interpolate(x, y, &self.velocity_x);
			let v = interpolation::bicubic_interpolate(x, y, &self.velocity_y);

			*p = integrators::euler(x, y, u, v, self.dt);
		}
	}

	pub fn advect(&mut self) {

		let u = self.velocity_x.clone();
		let v = self.velocity_y.clone();
		// self.velocity_x.advect(&u, &v, self.dt);
		// self.velocity_y.advect(&u, &v, self.dt);

		advection::semi_lagrangian(&mut self.velocity_x, &u, &v, self.dt, self.dx, &interpolation::bilinear_interpolate);
		advection::semi_lagrangian(&mut self.velocity_y, &u, &v, self.dt, self.dx, &interpolation::bilinear_interpolate);


		advection::upwind_advection(&mut self.density, &u, &v, self.dt, self.dx, &interpolation::bilinear_interpolate);
		//advection::semi_lagrangian(&mut self.density, &u, &v, self.dt, self.dx, &interpolation::bilinear_interpolate);

		//self.density.advect(&u, &v, self.dt);

		self.advect_particles();

		//swap(&mut self.velocity_x.values, &mut self.velocity_x.temp);
		//swap(&mut self.velocity_y.values, &mut self.velocity_y.temp);
		//swap(&mut self.density.values, &mut self.density.temp);
	}

	pub fn calculate_divergence(&mut self) {
		for i in 0..self.rows {
			for j in 0..self.columns {

				let x_velocity1 = if j < self.columns-1 { self.velocity_x.at(i, j+1) } else { 0.0 };
				let x_velocity2 = if j > 0 { self.velocity_x.at(i, j) } else { 0.0 };

				let y_velocity1 = if i < self.rows-1 { self.velocity_y.at(i + 1, j) } else { 0.0 };
				let y_velocity2 = if i > 0 { self.velocity_y.at(i, j) } else { 0.0 };

				*self.divergence.at_mut(i, j) = -(x_velocity1 - x_velocity2 + y_velocity1 - y_velocity2) / self.dx;
			}
		}
	}

	pub fn set_boundaries(&mut self) {
		for i in 0..self.rows {
			*self.velocity_x.at_mut(i, 0) = 0.0;
			*self.velocity_x.at_mut(i, self.columns) = 0.0;
		}
		for i in 0..self.columns {
			*self.velocity_y.at_mut(0, i) = 0.0;
			*self.velocity_y.at_mut(self.rows, i) = 0.0;
		}
	}

	pub fn project(&mut self) {
		self.calculate_divergence();
		self.pressure_solve();

		// apply pressure to velocity
		for i in 0..self.rows+1 {
			for j in 0..self.columns+1 {
				let p = i * self.columns + j;

				let p1 = if j < self.columns && i < self.rows { self.pressure.at(i, j) } else { 0.0 };
				let p2 = if i < self.rows && j < self.columns { self.pressure.at(i, j) } else { 0.0 };

				let p3 = if j as i32 - 1 >= 0 && i < self.rows { self.pressure.at(i, j - 1) } else { 0.0 };
				let p4 = if i as i32 - 1 >= 0 && j < self.columns { self.pressure.at(i - 1, j) } else { 0.0 };


				if j <= self.columns && i < self.rows {
					*self.velocity_x.at_mut(i, j) = self.velocity_x.at(i, j) - (self.dt / (self.fluid_density * self.dx)) * ( p1 - p3 );
				}
				if j < self.columns && i <= self.rows {
					*self.velocity_y.at_mut(i, j) = self.velocity_y.at(i, j) - (self.dt / (self.fluid_density * self.dx)) * ( p2 - p4 );
				}
			}
		}

		// boundary conditions
		self.set_boundaries();
	}

	pub fn add_source(&mut self, r: usize, c: usize, velocity_x: f64, velocty_y: f64, density: f64) {
		*self.velocity_x.at_mut(r, c + 1) = velocity_x;
		*self.velocity_x.at_mut(r, c) = velocity_x;
		*self.velocity_y.at_mut(r + 1, c) = velocty_y;
		*self.velocity_y.at_mut(r, c) = velocty_y;
		*self.density.at_mut(r, c) = density;
	}

	pub fn print_variable(v: &Variable) {
		for i in v.values.iter().rev() {
			for j in i {
				print!("{:.*} ", 2,  *j);
			}
			println!("");
		}
		println!("");
	}

	pub fn print_velocity(&self) {
		println!("{{");
		for i in 0..self.rows {
			for j in 0..self.columns {
				let v = format!("{}, {}", self.velocity_x.at(i, j), self.velocity_y.at(i, j) );
				let p = format!("{}, {}", j, i );

				print!("{{ {{ {} }}, {{ {} }} }}, ", p, v);
			}
		}
		println!("}}");
	}

	pub fn print_divergence(&self) {
		for i in (0..self.rows).rev() {
			for j in 0..self.columns {
				print!("{:.*}, ", 2, self.divergence.at(i, j) );
			}
			println!("");
		}
		println!("");
	}

	pub fn print_pressure(&self) {
		for i in (0..self.rows).rev() {
			for j in 0..self.columns {
				print!("{:.*}, ", 2, self.pressure.at(i, j));
			}
			println!("");
		}
		println!("");
	}

	pub fn write_velocity(&self) {
		let mut f = File::create("data.dat").unwrap();

		let mut data = String::new();

		for i in 0..self.rows {
			for j in 0..self.columns {
				let v = format!("{} {}", self.velocity_x.at(i, j), self.velocity_y.at(i, j) );
				let p = format!("{} {}", j, i );

				data = data + &*format!("{} {} ", p, v);
			}
		}

		let _ = f.write_all(data.as_bytes());
	}


	pub fn variable_image(v: &Variable, name: &str) {
		let mut temp = vec![];
		for i in v.values.iter().rev() {
			for j in i {
				temp.push(*j as u8);
				temp.push(*j as u8);
				temp.push(*j as u8);
				temp.push(*j as u8);
			}
		}
		//let _ = lodepng::encode_file(name, &temp, v.values[0].len() as usize, v.values.len() as usize, lodepng::LCT_RGB, 8);
		let _ = lodepng::encode32_file(name, &temp.as_slice(), v.values[0].len() as usize, v.values.len() as usize);
	}


}

// fn test() {
// 	let mut solver = FluidSolver::new(1.0, 100, 100, 0.01, 1.0);
// 	solver.solve();
//
// }
