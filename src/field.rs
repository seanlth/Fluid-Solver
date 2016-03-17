
#[derive(Clone)]
pub struct Field{
	pub field: Vec<f64>,
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

    fn next(&mut self) -> Option<f64> {
		let mut e = if self.current < self.field.len() {
			self.current += 1;
        	Some(self.field[self.current-1])
		}
		else {
			self.current = 0;
			None
		};

		e.clone()
    }
}


// impl<'a> Iterator for &'a mut Field {
//     type Item = f64;
//
//     fn next(&mut self) -> Option<f64> {
// 		let mut e = if self.current < self.field.len() {
// 			self.current += 1;
//         	Some(self.field[self.current-1])
// 		}
// 		else {
// 			self.current = 0;
// 			None
// 		};
//
// 		e.clone()
//     }
// }

// impl<'a> DoubleEndedIterator for &'a mut Field {
//
//     fn next_back(&mut self) -> Option<f64> {
// 		let e = if self.current < self.field.len() {
// 			self.current += 1;
//         	Some(self.field[self.field.len() - self.current])
// 		}
// 		else {
// 			self.current = 0;
// 			None
// 		};
//
// 		e.clone()
//     }
// }

// impl<'a> Iterator for core::iter::Rev<&Field> {
//     type Item = f64;
//
//     fn next(&mut self) -> Option<f64> {
// 		let e = if self.current < self.field.len() {
// 			self.current += 1;
//         	Some(self.field[self.current-1])
// 		}
// 		else {
// 			self.current = 0;
// 			None
// 		};
//
// 		e.clone()
//     }
// }
