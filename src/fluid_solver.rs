extern crate lodepng;

use std::mem::swap;

use linear_solvers;

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
	temp: Vec<Vec<f64>>,
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

		// adjust coordinates to relative to variable space
		x = x - self.offset_x;
		y = y - self.offset_y;

		if x == 0.5 && y == 4.5 {
		// println!("{}", x);
		// println!("{}", y);
		}

		// clamp coordinates within field
		x = self.clamp(x, 0.0, self.values[0].len() as f64 - 1.0);
		y = self.clamp(y, 0.0, self.values.len() as f64 - 1.0);

		// points to consider in interpolation
		// p1 - - - p2
		// |        |
		// |   p    |
		// |        |
		// p3 - - - p4
	    let (p1_x, p1_y) = ( (x+1.0).floor(), y.floor() );
	    let (p2_x, p2_y) = ( x.floor(), y.floor() );
	    let (p3_x, p3_y) = ( (x+1.0).floor(), (y+1.0).floor() );
	    let (p4_x, p4_y) = ( x.floor(), (y+1.0).floor() );

		//if x == 0.5 && y == 4.5 {
			// println!("{}, {}", p1_x, p1_y);
			// println!("{}, {}", p2_x, p2_y);
			// println!("{}, {}", p3_x, p3_y);
			// println!("{}, {}", p4_x, p4_y);
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

		let new_x = x as f64 - u_p * dt;
		let new_y = y as f64 - v_p * dt;

		//println!("{}, {} ", u_p, v_p);
		// println!("{}, {} ", x, y);

		(new_x, new_y)
	}

	// semi-lagrangian advection
	// see diagram __
	pub fn advect(&mut self, u: &Variable, v: &Variable, dt: f64) {
		let w = self.values[0].len();
		let h = self.values.len();

		for r in 0..h-1 {
			for c in 0..w-1 {
				// integrate from location of variable within grid
				let (old_x, old_y) = self.integrate(c as f64 + self.offset_x, r as f64 + self.offset_y, &u, &v, dt);

				//let (old_x, old_y) = (c as f64 + self.offset_x, r as f64 + self.offset_y);

				//println!("{}, {} -> {}, {}", old_x, old_y, c as f64 + self.offset_x, r as f64 + self.offset_y);
				self.temp[r][c] = self.interpolate_2d(old_x, old_y);
				//println!("{}", self.temp[r][c]);
				//println!("{}, {}", r, c);
			}
		}
	}
}


pub struct FluidSolver {
	pub velocity_x: Variable,
	velocity_y: Variable,
	pub density: Variable,
	divergence: Vec<f64>,
	pressure: Vec<f64>,
	fluid_density: f64,
	rows: i32,
	columns: i32,
	dt: f64,
	dx: f64
}

impl FluidSolver {
	pub fn new(fluid_density: f64, rows: i32, columns: i32, dt: f64, dx: f64) -> FluidSolver {
 		FluidSolver {
 			velocity_x: Variable::new_zeroed(rows, columns+1, 0.0, 0.5),
 			velocity_y: Variable::new_zeroed(rows+1, columns, 0.5, 0.0),
			density: Variable::new_zeroed(rows, columns, 0.5, 0.5),
			pressure: vec![0.0; (rows*columns) as usize],
			divergence: vec![0.0; (rows*columns) as usize],
			fluid_density: fluid_density,
 			rows: rows,
 			columns: columns,
			dt: dt,
			dx: dx
 		}
 	}



	//      0        1        2
	//   0  1  2  0  1  2  0  1  2
	//   0  1  2  3  4  5  6  7  8
	//
	//1  4 -1  0 -1  0  0  0  0  0
	//2 -1  4 -1  0 -1  0  0  0  0
	//3  0 -1  4  0  0 -1  0  0  0
	//1 -1  0  0  4 -1  0 -1  0  0
    //2  0 -1  0 -1  4 -1  0 -1  0
	//3  0  0 -1  0 -1  4  0  0 -1
	//1  0  0  0 -1  0  0  4 -1  0
	//2  0  0  0  0 -1  0 -1  4 -1
	//3  0  0  0  0  0 -1  0 -1  4

	pub fn laplacian(r: i32, c: i32, n: i32) -> f64 {
		let c_x = r % n;
		let c_y = r / n;

		let x = c % n;
		let y = c / n;

		if c_x == x && c_y == y { 4.0 }
		else if (c_x - x).abs() + (c_y - y).abs() == 1 { -1.0 }
		else { 0.0 }
	}

	// LP = D
	// see diagram
	pub fn pressure_solve(&mut self) {
		linear_solvers::gauss_seidel( FluidSolver::laplacian, &mut self.pressure, &self.divergence, self.rows*self.columns );
	}

	pub fn solve(&mut self) {
		self.advect();
		//self.project();
	}

	pub fn advect(&mut self) {

		let u = self.velocity_x.clone();
		let v = self.velocity_y.clone();
		self.velocity_x.advect(&u, &v, self.dt);
		self.velocity_y.advect(&u, &v, self.dt);
		self.density.advect(&u, &v, self.dt);

		swap(&mut self.velocity_x.values, &mut self.velocity_x.temp);
		swap(&mut self.velocity_y.values, &mut self.velocity_y.temp);
		swap(&mut self.density.values, &mut self.density.temp);
	}

	pub fn project(&mut self) {
		self.pressure_solve();

		for i in 1..self.rows-1 {
			for j in 1..self.columns-1 {
				let p = i * self.columns + j;
				self.velocity_x.values[i as usize][j as usize] = self.velocity_x.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( self.pressure[p as usize] - self.pressure[p as usize - 1] );
				self.velocity_y.values[i as usize][j as usize] = self.velocity_y.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( self.pressure[p as usize] - self.pressure[(p - self.columns) as usize] );
			}
		}

		// can be handled in pressure
		for i in 0..self.rows {
			self.velocity_x.values[i as usize][0] = 0.0;
			self.velocity_x.values[i as usize][self.columns as usize-1] = 0.0;
		}
		for i in 0..self.columns {
			self.velocity_x.values[i as usize][0] = 0.0;
			self.velocity_x.values[i as usize][self.rows as usize - 1] = 0.0;
		}
	}

	pub fn set_flow(&mut self, r: i32, c: i32, velocity_x: f64, velocty_y: f64, density: f64) {
		self.velocity_x.values[r as usize][c as usize] = velocity_x;
		self.velocity_y.values[r as usize][c as usize] = velocty_y;
		self.density.values[r as usize][c as usize] = density;
	}

	pub fn print_variable(v: &Variable) {
		for i in v.values.iter().rev() {
			for j in i {
				print!("{} ", *j);
			}
			println!("");
		}
	}

	pub fn density_image(&self) {
		let mut temp = vec![];
		for i in self.density.values.iter().rev() {
			for j in i {
				temp.push(*j as u8);
				temp.push(*j as u8);
				temp.push(*j as u8);
			}
		}
		lodepng::encode_file("density.png", &temp, self.columns as usize, self.rows as usize, lodepng::LCT_RGB, 8);
	}


}

// fn test() {
// 	let mut solver = FluidSolver::new(1.0, 100, 100, 0.01, 1.0);
// 	solver.solve();
//
// }
