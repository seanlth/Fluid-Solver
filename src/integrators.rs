use interpolation;

pub fn euler(x: f64, y: f64, u: f64, v: f64, dt: f64) -> (f64, f64) {

	let new_x = x as f64 + u * dt;
	let new_y = y as f64 + v * dt;

	(new_x, new_y)
}


// pub fn runge_kutta_4<F>(x: f64, y: f64, u: &Vec<Vec<f64>>, v: &Vec<Vec<f64>>, dt: f64, interpolator: &F) -> (f64, f64)
// 	where F : Fn(f64, f64, &Vec<Vec<f64>>) -> f64{
//
// 	let u_velocity = interpolator(x, y, &u);
// 	let v_velocity = interpolator(x, y, &v);
//
//     double midX = x - 0.5*timestep*firstU;
//     double midY = y - 0.5*timestep*firstV;
//
//         double midU = u.lerp(midX, midY)/_hx;
//         double midV = v.lerp(midX, midY)/_hx;
//
//         double lastX = x - 0.75*timestep*midU;
//         double lastY = y - 0.75*timestep*midV;
//
//         double lastU = u.lerp(lastX, lastY);
//         double lastV = v.lerp(lastX, lastY);
//
//         x -= timestep*((2.0/9.0)*firstU + (3.0/9.0)*midU + (4.0/9.0)*lastU);
//         y -= timestep*((2.0/9.0)*firstV + (3.0/9.0)*midV + (4.0/9.0)*lastV);
//
//
// 	let k1_x =
//
// }
