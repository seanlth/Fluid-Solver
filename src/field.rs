
struct Field {
	field: Vec<f64>,
	rows: usize,
	columns: usize,
	offset_x: f64,
	offset_y: f64
}

impl Field {
	pub fn new(rows: usize, columns: usize, offset_x: f64, offset_y: f64) -> Field {
		Field {
			field: vec![0.0; rows*columns],
			rows: rows,
			columns: columns,
			offset_x: offset_x,
			offset_y: offset_y
		}
	}

	pub fn at(&self, r: usize, c: usize) -> f64 {
		self.field[r * self.columns + c]
	}

	pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
		&mut self.field[r * self.columns + c]
	}

}
