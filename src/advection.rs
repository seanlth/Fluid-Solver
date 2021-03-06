
use field::Field;



// template advection
pub fn empty_advection(_: &mut Field, 
                       _: &Field, 
                       _: &Field, 
                       _: f64, _: f64, 
                       _: &Fn(f64, f64, &Field) -> f64, 
                       _: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {

}

// upwind with bicubic interpolation
pub fn upwind_advection(field: &mut Field, 
                        u: &Field, 
                        v: &Field, 
                        dt: f64, 
                        dx: f64, 
                        interpolator: &Fn(f64, f64, &Field) -> f64, 
                        _: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {
    let c = field.columns;
    let r = field.rows;

    let mut temp = field.clone();

    for i in 0..r {
        for j in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = j as f64 + field.offset_x;
            let y = i as f64 + field.offset_y;

            let u_velocity = interpolator(x, y, u) / dx;
            let v_velocity = interpolator(x, y, v) / dx;

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
pub fn semi_lagrangian(field: &mut Field, 
                       u: &Field, 
                       v: &Field, 
                       dt: f64, 
                       dx: f64, 
                       interpolator: &Fn(f64, f64, &Field) -> f64, 
                       integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {
    let c = field.columns;
    let r = field.rows;

    let mut temp = field.clone();

    for j in 0..r {
        for i in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = i as f64 + field.offset_x;
            let y = j as f64 + field.offset_y;

            let f1 = |o: f64, _: f64| -interpolator(o, y, &u)/dx;
            let f2 = |o: f64, _: f64| -interpolator(x, o, &v)/dx;

            let old_x = integrator(x, 0.0, &f1, dt);
            let old_y = integrator(y, 0.0, &f2, dt);

            // translate grid(old_x, old_y) -> field_array(i, j)
            *temp.at_fast_mut(j, i) = interpolator(old_x, old_y, field);
        }
    }

    *field = temp;
}

