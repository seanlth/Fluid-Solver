
use interpolation;
use integrators;

// upwind with bicubic interpolation
pub fn upwind_advection<F>(field: &mut Vec<Vec<f64>>, u: &Vec<Vec<f64>>, v: &Vec<Vec<f64>>, dt: f64, dx: f64, offset_x: f64, offset_y: f64, interpolator: &F)
 	where F : Fn(f64, f64, &Vec<Vec<f64>>) -> f64 {
	let c = field.len();
	let r = field[0].len();

	let mut temp = field.clone();

	for i in 0..r {
		for j in 0..c {

			let u_velocity = interpolator(j as f64 + offset_x, i as f64 + offset_y - 0.5, u);
			let v_velocity = interpolator(j as f64 + offset_x - 0.5, i as f64 + offset_y, v);

			let u_velocity_sign = u_velocity.signum();
			let v_velocity_sign = v_velocity.signum();

			let a = if j > 0 && j < c - 1 { (j as f64 - u_velocity_sign) as usize } else { j };
			let b = if i > 0 && i < r - 1 { (i as f64 - v_velocity_sign) as usize } else { i };

			temp[i][j] = field[i][j] - (dt / dx) * u_velocity * ( u_velocity_sign * field[i][j] - u_velocity_sign * field[i][a] )
									 - (dt / dx) * v_velocity * ( v_velocity_sign * field[i][j] - v_velocity_sign * field[b][j] );

		}
	}

	*field = temp;
}

// semi-lagrangian backtrace
pub fn semi_lagrangian<F>(field: &mut Vec<Vec<f64>>, u: &Vec<Vec<f64>>, v: &Vec<Vec<f64>>, dt: f64, dx: f64, offset_x: f64, offset_y: f64, interpolator: &F)
	where F : Fn(f64, f64, &Vec<Vec<f64>>) -> f64 {
	let c = field.len();
	let r = field[0].len();

	let mut temp = field.clone();

	for i in 0..c {
		for j in 0..r {
			let u_velocity = interpolator(j as f64 + offset_x, i as f64 + offset_y - 0.5, u);
			let v_velocity = interpolator(j as f64 + offset_x - 0.5, i as f64 + offset_y, v);

			// integrate from location of variable within grid
			let (old_x, old_y) = integrators::euler(j as f64 + offset_x, i as f64 + offset_y, -u_velocity, -v_velocity, dt);

			temp[i][j] = interpolator(old_x - offset_x, old_y - offset_y, field);
		}
	}

	*field = temp;
}
