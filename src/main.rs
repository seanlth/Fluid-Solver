extern crate Fluids;
extern crate lodepng;

use Fluids::fluid_solver::*;
use Fluids::linear_solvers;
use Fluids::visualiser::Visualiser;
use Fluids::interpolation::*;
use Fluids::field::Field;
use Fluids::advection::*;
use Fluids::integrators::*;

use std::io::prelude::*;
use std::fs::File;

use std::ops::Shr;


// fn test() {
//     let mut x: Vec<f64> = vec![0.0; 16];
//     let mut d: Vec<f64> = vec![0.0; 16];
//     let mut vx = vec![0.0; 20];
//     let mut vy = vec![10.0; 20];
//
//     d[0] = 10.0;
//     d[1] = 10.0;
//     d[2] = 10.0;
//     d[3] = 10.0;
//
//     vy[0] = 0.0;
//     vy[1] = 0.0;
//     vy[2] = 0.0;
//     vy[3] = 0.0;
//
//
//     println!("{:?}", vy);
//
//
//     linear_solvers::relaxation(&mut x, &d, 4, 4, 1.0, 0.01, 1.0, 400);
//
//     println!("{:?}", x);
//     println!("{:?}", d);
//
//     vy[0] = vy[0] - (0.01 / (1.0*1.0))*x[0];
//     vy[1] = vy[1] - (0.01 / (1.0*1.0))*x[1];
//     vy[2] = vy[2] - (0.01 / (1.0*1.0))*x[2];
//     vy[3] = vy[3] - (0.01 / (1.0*1.0))*x[3];
//
//     vy[4] = vy[4] - (0.01 / (1.0*1.0))*(x[4] - x[0]);
//     vy[5] = vy[5] - (0.01 / (1.0*1.0))*(x[5] - x[1]);
//     vy[6] = vy[6] - (0.01 / (1.0*1.0))*(x[6] - x[2]);
//     vy[7] = vy[7] - (0.01 / (1.0*1.0))*(x[7] - x[3]);
//
//     vy[8] = vy[8] - (0.01 / (1.0*1.0))*(x[8] - x[4]);
//     vy[9] = vy[9] - (0.01 / (1.0*1.0))*(x[9] - x[5]);
//     vy[10] = vy[10] - (0.01 / (1.0*1.0))*(x[10] - x[6]);
//     vy[11] = vy[11] - (0.01 / (1.0*1.0))*(x[11] - x[7]);
//
//     vy[12] = vy[12] - (0.01 / (1.0*1.0))*(x[12] - x[8]);
//     vy[13] = vy[13] - (0.01 / (1.0*1.0))*(x[13] - x[9]);
//     vy[14] = vy[14] - (0.01 / (1.0*1.0))*(x[14] - x[10]);
//     vy[15] = vy[15] - (0.01 / (1.0*1.0))*(x[15] - x[11]);
//
//     println!("{:?}", vy);
//
//
//     // for i in 0..self.rows+1 {
//     //     for j in 0..self.columns+1 {
//     //         let p = i * self.columns + j;
//     //
//     //         let p1 = if j < self.columns && i < self.rows { self.pressure[ p as usize ] } else { 0.0 };
//     //         let p2 = if i < self.rows && j < self.columns { self.pressure[ p as usize ] } else { 0.0 };
//     //
//     //         let p3 = if j - 1 >= 0 && i < self.rows { self.pressure[ p as usize - 1 ] } else { 0.0 };
//     //         let p4 = if i - 1 >= 0 && j < self.columns { self.pressure[ (p - self.columns) as usize ] } else { 0.0 };
//     //
//     //
//     //         if j <= self.columns && i < self.rows {
//     //             self.velocity_x.values[i as usize][j as usize] = self.velocity_x.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( p1- p3 );
//     //         }
//     //         if j < self.columns && i <= self.rows {
//     //             self.velocity_y.values[i as usize][j as usize] = self.velocity_y.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( p2 - p4 );
//     //         }
//     //     }
//     // }
//
//
// }

// fn test_interpolation() {
//     let mut f = File::create("data.dat").unwrap();
//
//     let mut data = String::new();
//
//     let mut values = vec![
//                       vec![1.0, 2.0, 4.0, 1.0],
//                       vec![6.0, 3.0, 5.0, 2.0],
//                       vec![4.0, 2.0, 1.0, 5.0],
//                       vec![5.0, 4.0, 2.0, 3.0],
//                      ];
//     //values.reverse();
//     for i in 0..75 {
//         data = data + "{";
//         for j in 0..75 {
//             if j < 75 - 1 {
//                 data = data + &*format!("{}, ", interpolation::bicubic_interpolate(j as f64 / 25.0, i as f64 / 25.0, &values));
//             }
//             else {
//                 data = data + &*format!("{}", interpolation::bicubic_interpolate(j as f64 / 25.0, i as f64 / 25.0, &values));
//             }
//         }
//         data = data + "}, ";
//     }
//
//     let _ = f.write_all(data.as_bytes());
//
//
// }


// pub fn runge_kutta_4<F>(x: f64, t: f64, f: F, dt: f64) -> f64
// 	where F : Fn(f64, f64) -> f64 {
//
//     let k1 = f(x, t);
//     let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
//     let k3 = f(x + (dt / 2.0)*k2, t + dt / 2.0);
//     let k4 = f(x + dt*k3, t + dt);
//
//     x + (k1 + 2.0*k2 + 2.0*k3 + k4) * (dt / 6.0)
// }
//
// pub fn euler<F>(x: f64, t: f64, f: F, dt: f64) -> f64
// 	where F : Fn(f64, f64) -> f64 {
//     x + f(x, t) * dt
// }
//
// pub fn bogacki_shampine<F>(x: f64, t: f64, f: F, dt: f64) -> f64
//     where F : Fn(f64, f64) -> f64 {
//
//     let k1 = f(x, t);
//     let k2 = f(x + (dt / 2.0)*k1, t + dt / 2.0);
//     let k3 = f(x + (3.0 * dt / 4.0)*k1, t + (3.0 * dt / 4.0));
//
//     x + (2.0 * k1 + 3.0 * k2 + 4.0 * k3) * (dt / 9.0)
// }

// fn test() {
//     let mut x = 1.0;
//
//     let f = |x: f64, t: f64| x;
//
//     let e: f64 = 2.7182818285;
//
//     let dt = 0.5;
//
//     print!("{}, ", x);
//
//     for i in 0..10 {
//         let t = i as f64 * dt;
//         //x = e.powf(t + dt);
//         //x = euler(x, t, &f, dt);
//         x = runge_kutta_4(x, t, &f, dt);
//         //println!("e^{} = {}, e^{} ~ {}", t, e.powf(t), t, x);
//         print!("{}, ", x);
//     }
//
// }

fn main() {
    // let mut answer = Vec::new();
    // linear_solvers::add_opencl_test(&vec![0.0, 1.0, 2.0, 3.0, 4.0], &vec![1.0, 2.0, 3.0, 4.0, 5.0], &mut answer);
    // println!("{:?}", answer);

    // test();

    //test_interpolation();

    //field_test();
    //let mut asd = Field::new(4, 4, 0.5, 0.5);
    //let asdasd = Field::new(4, 4, 0.5, 0.5);
    //linear_solvers::threaded_relaxation_unchecked(&mut asd, &asdasd, 0.1, 0.1, 1.0, 100);

    let mut solver = FluidSolver::new(1.0, 32, 32, 0.01, 1.0)
                                   .advection(semi_lagrangian)
                                   .interpolation(bilinear_interpolate)
                                   .integration(runge_kutta_4);

    //let visualiser = Visualiser::new();

    solver.apply_gravity();
    solver.calculate_divergence();
    // // solver.print_divergence();
    solver.pressure_solve();
    //solver.print_pressure();

    // println!("{:?}", None::<(usize, usize)>);


    // for i in 0..10000 {
    //
    //     //solver.add_source(5, 0, 1.0, 0.0, 0.1);
    //
    //      solver.add_source(62, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(63, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(64, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(65, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(66, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(67, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(68, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(69, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(70, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(71, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(72, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(73, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(74, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(75, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(76, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(77, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(78, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(79, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(80, 0, 500.0, 0.0, 0.1);
    //      solver.add_source(81, 0, 500.0, 0.0, 0.1);
    //
    //     //  solver.add_source(0, 62, 0.0, 500.0, 0.3);
    //     //  solver.add_source(0, 63, 0.0, 500.0, 0.3);
    //     //  solver.add_source(0, 64, 0.0, 500.0, 0.3);
    //     //  solver.add_source(0, 65, 0.0, 500.0, 0.3);
    //     //  solver.add_source(0, 66, 0.0, 500.0, 0.3);
    //
    //      solver.solve();
    //
    //      //visualiser.draw_density_image(&solver.density, &*format!("images/density{}.png", i));
    //      //visualiser.draw_density(&solver.density);
    //      visualiser.draw_density_rgb(&solver.density);
    //      //solver.print_density();
    //      //visualiser.draw_markers(&solver.particles, solver.columns, solver.rows);
    //      //visualiser.draw_pressure(&solver.pressure);
    // }


}
