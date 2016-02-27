

pub fn integrate(x: f64, y: f64, u: f64, v: f64, dt: f64) -> (f64, f64) {

	let new_x = x as f64 + u * dt;
	let new_y = y as f64 + v * dt;

	(new_x, new_y)
}
