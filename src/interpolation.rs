
use field::Field;

pub fn clamp(v: f64, a: f64, b: f64) -> f64 {
	if v < a { a }
	else if v > b { b }
	else { v }
}


// x and y must be corrected to grid
// field must be at least 8x8
pub fn bicubic_interpolate(mut x: f64, mut y: f64, field: &Field) -> f64 {
	let rows = field.rows;
	let columns = field.columns;

	x = clamp(x, 0.0, columns as f64 - 1.0);
	y = clamp(y, 0.0, rows as f64 - 1.0);


	// points to consider in interpolation
	// q1 - - - q2 - - - q3 - - - q4
	// |        |        |		  |
	// |        | 		 |		  |
	// |        |		 |		  |
	// q5 - - - p1 - - - p2 - - - q6
	// |        |        |		  |
	// |        | 	p	 |		  |
	// |        |		 |		  |
	// q7 - - - p3 - - - p4 - - - q8
	// |        |        |		  |
	// |        | 		 |		  |
	// |        |		 |		  |
	// q9 - - -q10 - - -q11 - - -q12

	let x1 = clamp((x-1.0).floor(), 0.0, columns as f64 - 1.0) as usize;
	let x2 = clamp((x).floor(), 0.0, columns as f64 - 1.0) as usize;
	let x3 = clamp((x+1.0).floor(), 0.0, columns as f64 - 1.0) as usize;
	let x4 = clamp((x+2.0).floor(), 0.0, columns as f64 - 1.0) as usize;

	let y1 = clamp((y-1.0).floor(), 0.0, rows as f64 - 1.0) as usize;
	let y2 = clamp((y).floor(), 0.0, rows as f64 - 1.0) as usize;
	let y3 = clamp((y+1.0).floor(), 0.0, rows as f64 - 1.0) as usize;
	let y4 = clamp((y+2.0).floor(), 0.0, rows as f64 - 1.0) as usize;

	let alpha = y - y2 as f64;
	let beta = x - x2 as f64;

	// interpolate across x-axis
	let a = cubic_interpolate(field.at(y1, x1), field.at(y1, x2), field.at(y1, x3), field.at(y1, x4), beta );
	let b = cubic_interpolate(field.at(y2, x1), field.at(y2, x2), field.at(y2, x3), field.at(y2, x4), beta );
	let c = cubic_interpolate(field.at(y3, x1), field.at(y3, x2), field.at(y3, x3), field.at(y3, x4), beta );
	let d = cubic_interpolate(field.at(y4, x1), field.at(y4, x2), field.at(y4, x3), field.at(y4, x4), beta );

	// interpolate across y-axis
	cubic_interpolate(a, b, c, d, alpha  )
}

fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {
    let mut a0 = d - c - a + b;
    let mut a1 = a - b - a0;
    let mut a2 = c - a;
    let mut a3 = b;

   	a0*w*w*w + a1*w*w + a2*w + a3
}

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
	a * w + b * (1.0 - w)
}


// interpolate across the field
// x and y must be corrected
//
pub fn bilinear_interpolate(mut x: f64, mut y: f64, field: &Vec<Vec<f64>>) -> f64 {

	// clamp coordinates within field
	x = clamp(x, 0.0, field[0].len() as f64 - 1.0);
	y = clamp(y, 0.0, field.len() as f64 - 1.0);

	// points to consider in interpolation
	// p4 - - - p3
	// |        |
	// |   p    |
	// |        |
	// p2 - - - p1
	let (p1_x, p1_y) = ( clamp((x+1.0).floor(), 0.0, field[0].len() as f64 - 1.0), clamp(y.floor(), 0.0, field.len() as f64 - 1.0) );
	let (p2_x, p2_y) = ( clamp(x.floor(), 0.0, field[0].len() as f64 - 1.0), clamp(y.floor(), 0.0, field.len() as f64 - 1.0) );
	let (p3_x, p3_y) = ( clamp((x+1.0).floor(), 0.0, field[0].len() as f64 - 1.0), clamp((y+1.0).floor(), 0.0, field.len() as f64 - 1.0) );
	let (p4_x, p4_y) = ( clamp(x.floor(), 0.0, field[0].len() as f64 - 1.0), clamp((y+1.0).floor(), 0.0, field.len() as f64 - 1.0) );


	// weight from 0 to 1 in x and y axis
	let alpha = y - p2_y;
	let beta = x - p2_x;

	let p1 = field[p1_y as usize][p1_x as usize];
	let p2 = field[p2_y as usize][p2_x as usize];
	let p3 = field[p3_y as usize][p3_x as usize];
	let p4 = field[p4_y as usize][p4_x as usize];

	// interpolate in x-axis
	let l1 = linear_interpolate(p1, p2, beta);
	let l2 = linear_interpolate(p3, p4, beta);

	// interpolate in y-axis
	linear_interpolate(l2, l1, alpha)
}
