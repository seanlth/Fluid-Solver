extern crate Fluids;

use Fluids::fluid_solver::Variable;

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

fn main() {
    let v = vec![vec![10.0, 20.0, 30.0, 40.0],
                 vec![50.0, 10.0, 90.0, 50.0],
                 vec![69.0, 54.0, 15.0, 10.0],
                 vec![98.0, 42.0, 87.0, 76.0]
                ];

    for p in v.iter().rev() {
        for q in p {
            print!("{} ", q);
        }
        println!("");
    }

    println!("{}", lerp((1.5, 2.0), &v ));


    let v = Variable::new(v, 0.0, 0.0);
    println!("{}", v.interpolate_2d(1.5, 2.0));

    //v.integrate()


}
