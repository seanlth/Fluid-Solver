
use field::Field;
use std::ops::Shr;



#[derive(Debug)]
struct V<T>(T);

impl<A, B, F> Shr<F> for V<A> where
    F: Fn(A) -> B {
	type Output = V<B>;

    fn shr(self, f: F) -> V<B> {
	let V(v) = self;
        V(f(v))
    }
}

pub fn clamp(v: f64, a: f64, b: f64) -> f64 {
    if v < a { a }
    else if v > b { b }
    else { v }
}

pub fn linear_interpolate(a: f64, b: f64, w: f64) -> f64 {
    a * w + b * (1.0 - w)
}

#[allow(dead_code)]
fn cubic_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {

    let V(minimum) = V(d) >> (|x| c.min(x)) >> (|x| b.min(x)) >> (|x| a.min(x));
    let V(maximum) = V(d) >> (|x| c.max(x)) >> (|x| b.max(x)) >> (|x| a.max(x));

    let a0 = d - c - a + b;
    let a1 = a - b - a0;
    let a2 = c - a;
    let a3 = b;

    let V(r) = V(a0*w*w*w + a1*w*w + a2*w + a3) >> (|x| minimum.max(x)) >> (|x| maximum.min(x));
    r
}

fn catmull_rom_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64 ) -> f64 {
    let V(minimum) = V(d) >> (|x| c.min(x)) >> (|x| b.min(x)) >> (|x| a.min(x));
    let V(maximum) = V(d) >> (|x| c.max(x)) >> (|x| b.max(x)) >> (|x| a.max(x));

    let a0 = -0.5*a + 1.5*b - 1.5*c + 0.5*d;
    let a1 = a - 2.5*b + 2.0*c - 0.5*d;
    let a2 = -0.5*a + 0.5*c;
    let a3 = b;

    let V(r) = V(a0*w*w*w + a1*w*w + a2*w + a3) >> (|x| minimum.max(x)) >> (|x| maximum.min(x));
    r
}

#[allow(dead_code)]
// tension : [-1, 1]
// bias : [-1, 1]
fn hermite_interpolate( a: f64, b: f64, c: f64, d: f64, w: f64, tension: f64, bias: f64 ) -> f64 {

    let mut m0  = (b-a)*(1.0+bias)*(1.0-tension)/2.0;
    m0 += (c-b)*(1.0-bias)*(1.0-tension)/2.0;
    let mut m1  = (c-b)*(1.0+bias)*(1.0-tension)/2.0;
    m1 += (d-c)*(1.0-bias)*(1.0-tension)/2.0;
    let a0 =  2.0*(w * w * w) - 3.0*(w * w) + 1.0;
    let a1 =    (w * w * w) - 2.0*(w * w) + w;
    let a2 =    (w * w * w) -   (w * w);
    let a3 = -2.0*(w * w * w) + 3.0*(w * w);

    a0 * b + a1 * m0 + a2 * m1 + a3 * c
}


pub fn empty_interpolate(_: f64, _: f64, _: &Field) -> f64 {
    0.0
}

// field must be at least 8x8
pub fn bicubic_interpolate(mut x: f64, mut y: f64, field: &Field) -> f64 {

    x = x - field.offset_x;
    y = y - field.offset_y;

    let rows = field.rows;
    let columns = field.columns;

    x = clamp(x, 0.0, columns as f64 - 1.0);
    y = clamp(y, 0.0, rows as f64 - 1.0);


    // points to consider in interpolation
    // q1 - - - q2 - - - q3 - - - q4
    // |        |        |		  |
    // |        | 		 |		  |
    // |        |		 |	      |
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
    let a = catmull_rom_interpolate(field.at_fast(y1, x1), field.at_fast(y1, x2), field.at_fast(y1, x3), field.at_fast(y1, x4), beta );
    let b = catmull_rom_interpolate(field.at_fast(y2, x1), field.at_fast(y2, x2), field.at_fast(y2, x3), field.at_fast(y2, x4), beta );
    let c = catmull_rom_interpolate(field.at_fast(y3, x1), field.at_fast(y3, x2), field.at_fast(y3, x3), field.at_fast(y3, x4), beta );
    let d = catmull_rom_interpolate(field.at_fast(y4, x1), field.at_fast(y4, x2), field.at_fast(y4, x3), field.at_fast(y4, x4), beta );

    // interpolate across y-axis
    catmull_rom_interpolate(a, b, c, d, alpha )
}



pub fn bilinear_interpolate(mut x: f64, mut y: f64, field: &Field) -> f64 {
    x = x - field.offset_x;
    y = y - field.offset_y;

    let rows = field.rows;
    let columns = field.columns;

    // clamp coordinates within field
    x = clamp(x, 0.0, columns as f64 - 1.0);
    y = clamp(y, 0.0, rows as f64 - 1.0);

    // points to consider in interpolation
    // p4 - - - p3
    // |        |
    // |   p    |
    // |        |
    // p2 - - - p1
    let (p1_x, p1_y) = ( clamp((x+1.0).floor(), 0.0, columns as f64 - 1.0), clamp(y.floor(), 0.0, rows as f64 - 1.0) );
    let (p2_x, p2_y) = ( clamp(x.floor(), 0.0, columns as f64 - 1.0), clamp(y.floor(), 0.0, rows as f64 - 1.0) );
    let (p3_x, p3_y) = ( clamp((x+1.0).floor(), 0.0, columns as f64 - 1.0), clamp((y+1.0).floor(), 0.0, rows as f64 - 1.0) );
    let (p4_x, p4_y) = ( clamp(x.floor(), 0.0, columns as f64 - 1.0), clamp((y+1.0).floor(), 0.0, rows as f64 - 1.0) );


    // weight from 0 to 1 in x and y axis
    let alpha = y - p2_y;
    let beta = x - p2_x;

    let p1 = field.at_fast(p1_y as usize, p1_x as usize);
    let p2 = field.at_fast(p2_y as usize, p2_x as usize);
    let p3 = field.at_fast(p3_y as usize, p3_x as usize);
    let p4 = field.at_fast(p4_y as usize, p4_x as usize);

    // interpolate in x-axis
    let l1 = linear_interpolate(p1, p2, beta);
    let l2 = linear_interpolate(p3, p4, beta);

    // interpolate in y-axis
    linear_interpolate(l2, l1, alpha)
}
