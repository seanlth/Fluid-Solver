extern crate Fluids;
extern crate lodepng;

use Fluids::fluid_solver::*;
use Fluids::linear_solvers;

//p1---p2
//|    |
//p3---p4
//


//p1 - - - p2
//|        |
//|   p    |
//|        |
//p3 - - - p4


fn lerp(pos: (f64, f64), v: &Vec<Vec<f64>>) -> f64 {
    let (x, y) = pos;
    let (p1_x, p1_y) = ( (x-1.0).ceil(), y.ceil() );
    let (p2_x, p2_y) = (x.ceil(), y.ceil());
    let (p3_x, p3_y) = ((x-1.0).ceil(), (y-1.0).ceil());
    let (p4_x, p4_y) = (x.ceil(), (y-1.0).ceil());

    let alpha = y - p3_y;
    let beta = x - p3_x;

    v[p1_y as usize][p1_x as usize] * (1.0-beta) * alpha + v[p2_y as usize][p2_x as usize] * beta * alpha + v[p4_y as usize][p3_x as usize] * (1.0-beta) * (1.0-alpha) + v[p4_y as usize][p4_x as usize] * beta * (1.0 - alpha)
}

pub fn jacobi(A: fn(i32, i32) -> f64, x: fn(i32) -> f64, b: fn(i32) -> f64, r: &mut Vec<f64>, n: i32) {

    let mut temp1: Vec<f64> = std::vec::from_elem(0.0, n as usize);
    let mut temp2: Vec<f64> = std::vec::from_elem(0.0, n as usize);
    let mut diff: Vec<f64> = std::vec::from_elem(0.0, n as usize);

    let zero: Vec<f64> = std::vec::from_elem(0.0, n as usize);

    for i in 0..n {
        temp1[i as usize] = x(i);
    }

    let limit = 100;
    let epsilon = 0.001;

    for k in 0..limit {
        let mut a: f64 = 0.0;
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if i != j {
                    sigma = sigma + A(i, j) * temp1[j as usize];
                }

                diff[i as usize] += A(i, j) * temp1[j as usize];
            }
            temp2[i as usize] = ( b(i) - sigma ) / A(i, i);
            diff[i as usize] -= b(i);
            a += diff[i as usize];
        }

        if a.abs().sqrt() < epsilon {
             break;
        }

        temp1 = temp2.clone();
        diff = zero.clone();
    }

    *r = temp2;
}


// fn A(r: i32, c: i32) -> f64 {
//     let t = vec![vec![2.0, 1.0],
//                  vec![5.0, 7.0]
//                 ];
//     t[r as usize][c as usize]
// }
//
// fn x(c: i32) -> f64 {
//     let t: Vec<f64> = vec![0.0, 1.0];
//     t[c as usize]
// }
//
// fn b(c: i32) -> f64 {
//     let t: Vec<f64> = vec![11.0, 13.0];
//     t[c as usize]
// }


fn A(r: i32, c: i32, n: i32) -> f64 {
    let t = vec![vec![12.0, -9.0],
                 vec![8.0, 9.0]
                ];
    t[r as usize][c as usize]
}

fn x(c: i32) -> f64 {
    let t: Vec<f64> = vec![1.0, 1.0];
    t[c as usize]
}

fn b(c: i32) -> f64 {
    let t: Vec<f64> = vec![37.0, 23.0];
    t[c as usize]
}


fn main() {

    let mut solver = FluidSolver::new(1.0, 101, 101, 0.01, 0.1);


    // solver.set_flow(5, 0, 100.0, 0.0, 0.0);
    // solver.set_flow(5, 1, 100.0, 0.0, 0.0);
    // solver.set_flow(5, 2, 100.0, 0.0, 0.0);



    solver.add_source(2, 5, 700.0, 0.0, 0.0);

    // for i in 0..11 {
    //     for j in 0..11 {
    //         solver.add_source(i, j, 500.0, 0.0, 0.0);
    //     }
    // }


    for i in 0..1000 {
        solver.solve();
        //FluidSolver::print_variable(&solver.velocity_x);
    }
    //solver.print_velocity();
    //solver.calculate_divergence();
    //solver.print_divergence();
    //solver.project();


    // solver.print_pressure();
    //solver.calculate_divergence();
    //solver.print_divergence();

    // solver.solve();
    //solver.calculate_divergence();
    //solver.print_divergence();
    //FluidSolver::print_variable(&solver.velocity_x);

    //solver.print_velocity();

    solver.write_velocity();

    //solver.solve();
    //solver.solve();
    // solver.solve();
    // solver.solve();
    // solver.solve();
    // solver.solve();
    // solver.solve();
    // solver.solve();
    //solver.solve();


    //FluidSolver::print_variable(&solver.velocity_x);
    //

    // FluidSolver::variable_image(&solver.velocity_x, "velocity_x");
    // FluidSolver::variable_image(&solver.velocity_y, "velocity_y");


    //let mut result: Vec<f64> = vec![0.0, 0.0];

    //let _ = linear_solvers::jacobi(A, &mut result, b, 2);
    //println!("{}, {}", result[0], result[1]);

    // for i in 0..16 {
    //     for j in 0..16 {
    //         let f = FluidSolver::laplacian(i, j, 4);
    //         print!("{} ", f);
    //     }
    //     println!("");
    // }



}
