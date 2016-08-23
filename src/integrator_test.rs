pub fn runge_kutta_4<F>(x: f64, t: f64, f: F, dt: f64) -> f64
	where F : Fn(f64, f64) -> f64 {

    let k1 = f(x, t);
    let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
    let k3 = f(x + (dt / 2.0)*k2, t + dt / 2.0);
    let k4 = f(x + dt*k3, t + dt);

    x + (k1 + 2.0*k2 + 2.0*k3 + k4) * (dt / 6.0)
}

pub fn euler<F>(x: f64, t: f64, f: F, dt: f64) -> f64
	where F : Fn(f64, f64) -> f64 {
    x + f(x, t) * dt
}

pub fn bogacki_shampine<F>(x: f64, t: f64, f: F, dt: f64) -> f64
    where F : Fn(f64, f64) -> f64 {

    let k1 = f(x, t);
    let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
    let k3 = f(x + (3.0 * dt / 4.0)*k1, t + (3.0 * dt / 4.0));

    x + (2.0 * k1 + 3.0 * k2 + 4.0 * k3) * (dt / 9.0)
}

fn test() {
    let mut x = 1.0;

    let f = |x: f64, t: f64| x;

    let e: f64 = 2.7182818285;

    let dt = 0.5;

    print!("{}, ", x);

    for i in 0..10 {
        let t = i as f64 * dt;
        //x = e.powf(t + dt);
        //x = euler(x, t, &f, dt);
        x = runge_kutta_4(x, t, &f, dt);
        //println!("e^{} = {}, e^{} ~ {}", t, e.powf(t), t, x);
        print!("{}, ", x);
    }

}
