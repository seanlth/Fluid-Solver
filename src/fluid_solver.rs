




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

	pub fn interpolate_1d(&self, a: f64, b: f64, w: f64) -> f64 {
		a * (1.0 - w) + b * w
	}

	pub fn interpolate_2d(&self, x: f64, y: f64) -> f64 {
	    let (p1_x, p1_y) = ( (x-1.0).ceil(), y.ceil() );
	    let (p2_x, p2_y) = (x.ceil(), y.ceil());
	    let (p3_x, p3_y) = ((x-1.0).ceil(), (y-1.0).ceil());
	    let (p4_x, p4_y) = (x.ceil(), (y-1.0).ceil());

	    let alpha = y - p3_y;
	    let beta = x - p3_x;

		let l1 = self.interpolate_1d(self.values[p1_y as usize][p1_x as usize], self.values[p2_y as usize][p2_x as usize], beta);
		let l2 = self.interpolate_1d(self.values[p3_y as usize][p3_x as usize], self.values[p4_y as usize][p4_x as usize], beta);

		self.interpolate_1d(l2, l1, alpha)
	}

	// back trace
	pub fn integrate(&self, p: (f64, f64), u: f64, v: f64, dt: f64) -> (f64, f64) {
		let (mut x, mut y) = p;

		x = x - u*dt;
		y = y - v*dt;

		(x, y)
	}

	pub fn advect(&self, u: &Variable, v: &Variable, dt: f64) {
		let asd = vec![10, 20];

	}

}


// pub struct FluidSolver {
// 	pressure: Vec<Vec<f64>>,
// 	velocity_x: Vec<Vec<f64>>,
// 	velocity_y: Vec<Vec<f64>>,
// 	rows: i32,
// 	columns: i32,
// }
//
// impl FluidSolver {
// 	pub fn new(rows: i32, columns: i32) -> FluidSolver {
//
//
//
//
// 		FluidSolver {
// 			pressure: Vec::with_capacity( ( rows * columns ) as usize ),
// 			velocity_x: Vec::with_capacity( ( rows * (columns+1) ) as usize ),
// 			velocity_y: Vec::with_capacity( ( (rows+1) * columns ) as usize ),
// 			rows: rows,
// 			columns: columns
// 		}
// 	}
//
// 	pub fn interpolate(&self, x: f64, y: f64) -> f64 {
// 		1.0
// 	}
//
// 	pub fn euler(&self, )
//
// 	// pub fn pressure_at(&self, row: i32, column: i32) -> f64 {
// 	// 	self.pressure[ (row * self.columns + column) as usize ]
// 	// }
// 	//
// 	// pub fn velocity_at(&self, row: i32, column: i32) -> (f64, f64) {
// 	// 	(self.velocity_x[ (row * self.columns + column) as usize ], self.velocity_y[ (row * self.columns + column) as usize ])
// 	// }
//
// 	pub fn advect(&self, row: i32, column: u32, quantity: f64) -> f64 {
// 		1.0
// 	}
//
//
//
// }
