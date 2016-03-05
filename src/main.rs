extern crate Fluids;
extern crate lodepng;

use Fluids::fluid_solver::*;
use Fluids::linear_solvers;
use Fluids::visualiser::Visualiser;

fn test() {
    let mut x: Vec<f64> = vec![0.0; 16];
    let mut d: Vec<f64> = vec![0.0; 16];
    let mut vx = vec![0.0; 20];
    let mut vy = vec![10.0; 20];

    d[0] = 10.0;
    d[1] = 10.0;
    d[2] = 10.0;
    d[3] = 10.0;

    vy[0] = 0.0;
    vy[1] = 0.0;
    vy[2] = 0.0;
    vy[3] = 0.0;


    println!("{:?}", vy);


    linear_solvers::relaxation(&mut x, &d, 4, 4, 1.0, 0.01, 1.0, 400);

    println!("{:?}", x);
    println!("{:?}", d);

    vy[0] = vy[0] - (0.01 / (1.0*1.0))*x[0];
    vy[1] = vy[1] - (0.01 / (1.0*1.0))*x[1];
    vy[2] = vy[2] - (0.01 / (1.0*1.0))*x[2];
    vy[3] = vy[3] - (0.01 / (1.0*1.0))*x[3];

    vy[4] = vy[4] - (0.01 / (1.0*1.0))*(x[4] - x[0]);
    vy[5] = vy[5] - (0.01 / (1.0*1.0))*(x[5] - x[1]);
    vy[6] = vy[6] - (0.01 / (1.0*1.0))*(x[6] - x[2]);
    vy[7] = vy[7] - (0.01 / (1.0*1.0))*(x[7] - x[3]);

    vy[8] = vy[8] - (0.01 / (1.0*1.0))*(x[8] - x[4]);
    vy[9] = vy[9] - (0.01 / (1.0*1.0))*(x[9] - x[5]);
    vy[10] = vy[10] - (0.01 / (1.0*1.0))*(x[10] - x[6]);
    vy[11] = vy[11] - (0.01 / (1.0*1.0))*(x[11] - x[7]);

    vy[12] = vy[12] - (0.01 / (1.0*1.0))*(x[12] - x[8]);
    vy[13] = vy[13] - (0.01 / (1.0*1.0))*(x[13] - x[9]);
    vy[14] = vy[14] - (0.01 / (1.0*1.0))*(x[14] - x[10]);
    vy[15] = vy[15] - (0.01 / (1.0*1.0))*(x[15] - x[11]);

    println!("{:?}", vy);


    // for i in 0..self.rows+1 {
    //     for j in 0..self.columns+1 {
    //         let p = i * self.columns + j;
    //
    //         let p1 = if j < self.columns && i < self.rows { self.pressure[ p as usize ] } else { 0.0 };
    //         let p2 = if i < self.rows && j < self.columns { self.pressure[ p as usize ] } else { 0.0 };
    //
    //         let p3 = if j - 1 >= 0 && i < self.rows { self.pressure[ p as usize - 1 ] } else { 0.0 };
    //         let p4 = if i - 1 >= 0 && j < self.columns { self.pressure[ (p - self.columns) as usize ] } else { 0.0 };
    //
    //
    //         if j <= self.columns && i < self.rows {
    //             self.velocity_x.values[i as usize][j as usize] = self.velocity_x.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( p1- p3 );
    //         }
    //         if j < self.columns && i <= self.rows {
    //             self.velocity_y.values[i as usize][j as usize] = self.velocity_y.values[i as usize][j as usize] - (self.dt / (self.fluid_density * self.dx)) * ( p2 - p4 );
    //         }
    //     }
    // }


}


fn main() {


    let mut solver = FluidSolver::new(1.0, 5, 5, 0.01, 1.0);

    let visualiser = Visualiser::new();


    // solver.pressure = vec![
    //                        vec![-19.60, -19.60, -19.60, -19.60, -19.60],
    //                        vec![-9.80, -9.80, -9.80, -9.80, -9.80],
    //                        vec![0.00, 0.00, 0.00, 0.00, 0.00],
    //                        vec![9.80, 9.80, 9.80, 9.80, 9.80],
    //                        vec![19.60, 19.60, 19.60, 19.60, 19.60]
    //                       ];

    solver.pressure = vec![4.9, 4.9, 4.9, 4.9, 4.9,
                           14.7, 14.7, 14.7, 14.7, 14.7,
                           24.5, 24.5, 24.5, 24.5, 24.5,
                           34.3, 34.3, 34.3, 34.3, 34.3,
                           44.1, 44.1, 44.1, 44.1, 44.1];

    solver.pressure.reverse();

    solver.print_pressure();

    for i in 0..1 {
        //solver.add_source(125, 64, 0.0, -500.0, 1.0);

        solver.solve();

        // if i % 10 == 0 {
        //     let f = format!("./images/density{}.png", i / 10);
        //
        //     FluidSolver::variable_image(&solver.density, &*f);
        // }


        //visualiser.draw_density(&solver.density.values);
        //visualiser.draw_markers(&solver.particles, solver.columns, solver.rows);
    }
    solver.print_pressure();

    FluidSolver::print_variable(&solver.velocity_y);

}
