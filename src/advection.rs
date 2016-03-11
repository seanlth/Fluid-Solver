
use interpolation;
use integrators;
use field::Field;

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
pub fn semi_lagrangian<I>(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &I)
	where I : Fn(f64, f64, &Field) -> f64 {
	let c = field.columns;
	let r = field.rows;

	let mut temp = field.clone();

	for j in 0..r {
		for i in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = j as f64 + field.offset_x;
            let y = i as f64 + field.offset_y;

            // position relative to velocity array, grid(x, y) -> velocity(i, j)
            // finds the velocity at that point
			let u_velocity = interpolator(x - u.offset_x, y - u.offset_y, u);
			let v_velocity = interpolator(x - v.offset_x, y - v.offset_y, v);

			// integrate from location of variable within grid
			let (old_x, old_y) = integrators::euler(x, y, -u_velocity, -v_velocity, dt);

            // translate grid(old_x, old_y) -> field_array(i, j)
			*temp.at_fast_mut(j, i) = interpolator(old_x - field.offset_x, old_y - field.offset_y, field);
		}
	}

	*field = temp;
}
