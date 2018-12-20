
pub fn empty(_: f64, _: f64, _: &Fn(f64, f64) -> f64, _: f64) -> f64 {
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
