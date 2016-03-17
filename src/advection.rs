
use interpolation;
use integrators;
use field::Field;

// template advection
pub fn empty_advection(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64) {

}

// upwind with bicubic interpolation
pub fn upwind_advection<F>(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &F)
 	where F : Fn(f64, f64, &Field) -> f64 {
	let c = field.columns;
	let r = field.rows;

	let mut temp = field.clone();

	for i in 0..r {
		for j in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = j as f64 + field.offset_x;
            let y = i as f64 + field.offset_y;

			let u_velocity = interpolator(x, y, u);
			let v_velocity = interpolator(x, y, v);

			let u_velocity_sign = u_velocity.signum();
			let v_velocity_sign = v_velocity.signum();

			let a = if j > 0 && j < c - 1 { (j as f64 - u_velocity_sign) as usize } else { j };
			let b = if i > 0 && i < r - 1 { (i as f64 - v_velocity_sign) as usize } else { i };

			*temp.at_mut(i, j) = field.at(i, j) - (dt / dx) * u_velocity * ( u_velocity_sign * field.at(i, j) - u_velocity_sign * field.at(i, a) )
									 - (dt / dx) * v_velocity * ( v_velocity_sign * field.at(i, j) - v_velocity_sign * field.at(b, j) );

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
            let x = i as f64 + field.offset_x;
            let y = j as f64 + field.offset_y;

            // position relative to velocity array, grid(x, y) -> velocity(i, j)
            // finds the velocity at that point
	        //let u_velocity = interpolator(x, y, u);
		    //let v_velocity = interpolator(x, y, v);

			// integrate from location of variable within grid
			//let (old_x, old_y) = integrators::euler(x, y, -u_velocity, -v_velocity, dt);
            let f1 = |_: f64, _: f64| -interpolation::bicubic_interpolate(x, y, &u);
            let f2 = |_: f64, _: f64| -interpolation::bicubic_interpolate(x, y, &v);
            //
            let old_x = integrators::runge_kutta_4(x, 0.0, f1, dt);
            let old_y = integrators::runge_kutta_4(y, 0.0, f2, dt);

            // translate grid(old_x, old_y) -> field_array(i, j)
			*temp.at_fast_mut(j, i) = interpolator(old_x, old_y, field);
		}
	}

	*field = temp;
}
