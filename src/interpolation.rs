
pub fn clamp(&self, v: f64, a: f64, b: f64) -> f64 {
	if v < a { a }
	else if v > b { b }
	else { v }
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
