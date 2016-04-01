
#[derive(Clone)]
pub struct Field{
	pub field: Vec<f64>,
	pub rows: usize,
	pub columns: usize,
	pub offset_x: f64,
	pub offset_y: f64,
}

impl Field {
	pub fn new(rows: usize, columns: usize, offset_x: f64, offset_y: f64) -> Field {
		Field {
			field: vec![0.0; rows*columns],
			rows: rows,
			columns: columns,
			offset_x: offset_x,
			offset_y: offset_y,
		}
	}

	pub fn at(&self, r: usize, c: usize) -> f64 {
		self.field[r * self.columns + c]
	}

	pub fn at_mut(&mut self, r: usize, c: usize) -> &mut f64 {
		&mut self.field[r * self.columns + c]
	}

	pub fn at_fast(&self, r: usize, c: usize) -> f64 {
		unsafe {
	        *self.field.get_unchecked(r * self.columns + c)
	    }
	}

	pub fn at_fast_mut(&mut self, r: usize, c: usize) -> &mut f64 {
		unsafe {
	        self.field.get_unchecked_mut(r * self.columns + c)
	    }
	}
}
