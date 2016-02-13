use std::mem::swap;



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



pub struct Variable {
	values: Vec<Vec<f64>>,
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
		a * (1.0 - w) + b * w
	}

	pub fn clamp(&self, v: f64, a: f64, b: f64) -> f64 {
		if v < a { a }
		else if v > b { b }
		else { v }
	}


	// interpolate across the field
	pub fn interpolate_2d(&self, mut x: f64, mut y: f64) -> f64 {

		// clamp coordinates within field
		x = self.clamp(x, self.offset_x, self.values[0].len() as f64);
		y = self.clamp(y, self.offset_y, self.values.len() as f64);

		// points to consider in interpolation
		// p1 - - - p2
		// |        |
		// |   p    |
		// |        |
		// p3 - - - p4
	    let (p1_x, p1_y) = ( (x-1.0).ceil(), y.ceil() );
	    let (p2_x, p2_y) = (x.ceil(), y.ceil());
	    let (p3_x, p3_y) = ((x-1.0).ceil(), (y-1.0).ceil());
	    let (p4_x, p4_y) = (x.ceil(), (y-1.0).ceil());

 		// weight from 0 to 1 in x and y axis
	    let alpha = y - p3_y;
	    let beta = x - p3_x;

		// interpolate in x-axis
		let l1 = self.interpolate_1d(self.values[p1_y as usize][p1_x as usize], self.values[p2_y as usize][p2_x as usize], beta);
		let l2 = self.interpolate_1d(self.values[p3_y as usize][p3_x as usize], self.values[p4_y as usize][p4_x as usize], beta);

		// interpolate in y-axis
		self.interpolate_1d(l2, l1, alpha)
	}

	// forward euler back trace
	// see diagram __
	pub fn integrate(&self, x: f64, y: f64, u: &Variable, v: &Variable, dt: f64) -> (f64, f64) {
		let new_x = x - u.values[y as usize][x as usize] * dt;
		let new_y = y - v.values[y as usize][x as usize] * dt;

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
				let (old_x, old_y) = self.integrate(r as f64 + self.offset_x, c as f64 + self.offset_y, &u, &v, dt);
				self.temp[r][c] = self.interpolate_2d(old_x, old_y);
			}
		}
	}
}


pub struct FluidSolver {
	pressure: Variable,
	velocity_x: Variable,
	velocity_y: Variable,
	density: Variable,
	rows: i32,
	columns: i32,
	dt: f64,
	dx: f64
}

impl FluidSolver {
	pub fn new(rows: i32, columns: i32, dt: f64, dx: f64) -> FluidSolver {
 		FluidSolver {
 			pressure: Variable::new_zeroed(rows, columns, 0.5, 0.5),
 			velocity_x: Variable::new_zeroed(rows, columns, 0.5, 0.5),
 			velocity_y: Variable::new_zeroed(rows, columns, 0.5, 0.5),
			density: Variable::new_zeroed(rows, columns, 0.5, 0.5),
 			rows: rows,
 			columns: columns,
			dt: dt,
			dx: dx
 		}
 	}

	pub fn solve(&mut self) {

	}

	pub fn jacobi(&mut self) {
		let limit = 200;

		let epsilon = 0.001;
		let mut a = 0.0;

		for k in 0..limit {
			for i in 0..self.rows {
				for j in 0..self.columns {

				}
			}

			if a < epsilon {
				break;
			}
		}

	}

	// LP = D
	// see diagram
	pub fn pressure_solve(&mut self) {

	}

}
