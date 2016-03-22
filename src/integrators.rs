use interpolation;
use field::Field;

pub fn empty(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64 {
    0.0
}

// Order 1
pub fn euler(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64 {
    x + f(x, t) * dt
}

// Order 3
pub fn bogacki_shampine(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64 {

    let k1 = f(x, t);
    let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
    let k3 = f(x + (3.0 * dt / 4.0)*k1, t + (3.0 * dt / 4.0));

    x + (2.0 * k1 + 3.0 * k2 + 4.0 * k3) * (dt / 9.0)
}

// Order 4
pub fn runge_kutta_4(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64 {

    let k1 = f(x, t);
    let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
    let k3 = f(x + (dt / 2.0)*k2, t + dt / 2.0);
    let k4 = f(x + dt*k3, t + dt);

    x + (k1 + 2.0*k2 + 2.0*k3 + k4) * (dt / 6.0)
}




// pub fn euler(x: f64, y: f64, u: f64, v: f64, dt: f64) -> (f64, f64) {
//
// 	let new_x = x as f64 + u * dt;
// 	let new_y = y as f64 + v * dt;
//
// 	(new_x, new_y)
// }

// pub fn bogacki_shampine<F>(x: f64, y: f64, u: &Field, v: &Field, dt: f64, interpolator: &F) -> (f64, f64)
//     where F : Fn(f64, f64, &Field) -> f64 {
//
//     let f = |x: f64, y, t: f64| -> (f64, f64) {  (interpolator(x, y, &u), interpolator(x, y, &v))  };
//
//     let k1_x
//
//     double midX = x + 0.5*timestep * u.lerp(x, y)/_hx;
//     double midY = y + 0.5*timestep * v.lerp(x, y)/_hx;
//
//     double lastX = x + 0.75*timestep * u.lerp(midX, midY)/_hx;
//     double lastY = y + 0.75*timestep * v.lerp(midX, midY)/_hx;
//
//     double lastU = u.lerp(lastX, lastY);
//     double lastV = v.lerp(lastX, lastY);
//
//     x += timestep*((2.0/9.0)*firstU + (3.0/9.0)*midU + (4.0/9.0)*lastU);
//     y += timestep*((2.0/9.0)*firstV + (3.0/9.0)*midV + (4.0/9.0)*lastV);
//
// }


// pub fn runge_kutta_4<F>(x: f64, y: f64, u: &Field, v: &Field, dt: f64, interpolator: &F) -> (f64, f64)
// 	where F : Fn(f64, f64, &Field) -> f64 {
//
//     let f = |x: f64, y, t: f64| -> (f64, f64) {  (x + interpolator(x, y, &u)*t, y + interpolator(x, y, &v)*t)  };
//
//     let (k1_x, k1_y) = f(x, y, 0.0);
//     let (k2_x, k2_y) = f(x + (dt / 2.0)*k1_x, y + (dt / 2.0)*k1_y, dt / 2.0);
//     let (k3_x, k3_y) = f(x + (dt / 2.0)*k2_x, y + (dt / 2.0)*k2_y, dt / 2.0);
//     let (k4_x, k4_y) = f(x + dt*k3_x, y + dt*k3_y, dt / 2.0);
//
//     (x + (k1_x + 2.0*k2_x + 2.0*k3_x + k4_x) / 6.0, y + (k1_y + 2.0*k2_y + 2.0*k3_y + k4_y) / 6.0)
// }
