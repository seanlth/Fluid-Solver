
#[derive(Clone)]
pub struct Field{
	field: Vec<f64>,
	pub rows: usize,
	pub columns: usize,
	pub offset_x: f64,
	pub offset_y: f64,
	current: usize,
}

impl Field {
	pub fn new(rows: usize, columns: usize, offset_x: f64, offset_y: f64) -> Field {
		Field {
			field: vec![0.0; rows*columns],
			rows: rows,
			columns: columns,
			offset_x: offset_x,
			offset_y: offset_y,
			current: 0
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

impl Iterator for Field {
    type Item = f64;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<f64> {
		let e = if self.current < self.field.len() {
        	Some(self.field[self.current])
		}
		else {
			None
		};

		self.current += 1;
		e.clone()
    }
}
