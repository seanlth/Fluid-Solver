
// let scale = self.dt/(self.fluid_density*self.dx);
//
// for y in 0..self.rows {
// 	let mut idx = 0;
// 	for x in 0..self.columns {
//
// 		self.velocity_x.values[y as usize][x as usize] = self.velocity_x.values[y as usize][x as usize] - scale*self.pressure[idx];
//         self.velocity_x.values[y as usize][x as usize+1]  = self.velocity_x.values[y as usize][x as usize+1] + scale*self.pressure[idx];
//         self.velocity_y.values[y as usize][x as usize]  = self.velocity_y.values[y as usize][x as usize] - scale*self.pressure[idx];
//         self.velocity_y.values[y as usize+1][x as usize]  += scale*self.pressure[idx];
// 		idx = idx + 1;
// 	}
// }
//
// for y in 0..self.rows {
// 	self.velocity_x.values[y as usize][0] = 0.0;
// 	self.velocity_x.values[y as usize][self.columns as usize] = 0.0;
// }
// for x in 0..self.columns {
// 	self.velocity_y.values[0][x as usize] = 0.0;
// 	self.velocity_x.values[self.rows as usize][x as usize] = 0.0;
// }








// pub fn jacobi(a: fn(i32, i32) -> f64, x: fn(i32) -> f64, b: fn(i32) -> f64, r: &mut Vec<f64>, n: i32) {
//
//     let mut temp1: Vec<f64> = vec![0.0; n as usize];
//     let mut temp2: Vec<f64> = vec![0.0; n as usize];
//     let mut diff: Vec<f64> = vec![0.0; n as usize];
//     let zero: Vec<f64> = vec![0.0; n as usize];
//
//     for i in 0..n {
//         temp1[i as usize] = x(i);
//     }
//
//     let limit = 100;
//     let epsilon = 0.001;
//
//     for _ in 0..limit {
//         let mut s: f64 = 0.0;
//         for i in 0..n {
//             let mut sigma = 0.0;
//             for j in 0..n {
//                 if i != j {
//                     sigma = sigma + a(i, j) * temp1[j as usize];
//                 }
//
//                 diff[i as usize] += a(i, j) * temp1[j as usize];
//             }
//             temp2[i as usize] = ( b(i) - sigma ) / a(i, i);
//             diff[i as usize] -= b(i);
//             s += diff[i as usize];
//         }
//
//         if s.abs().sqrt() < epsilon {
//              break;
//         }
//
//         temp1 = temp2.clone();
//         diff = zero.clone();
//     }
//
//     *r = temp2;
// }

pub fn relax(a: fn(i32, i32, i32) -> f64, p: &mut Vec<f64>, b: &Vec<f64>, w: i32, h: i32) {
    let scale = 0.01;
    let limit = 100;

    for k in 0..limit {
        for y in 0..h {
            for x in 0..w {
                let idx = x + y*w;

                let mut diag = 0.0;
                let mut offDiag = 0.0;

                if (x > 0) {
                    diag    += scale;
                    offDiag -= scale*p[idx as usize - 1];
                }
                if (y > 0) {
                    diag    += scale;
                    offDiag -= scale*p[idx as usize - w as usize];
                }
                if (x < w - 1) {
                    diag    += scale;
                    offDiag -= scale*p[idx as usize + 1];
                }
                if (y < h - 1) {
                    diag    += scale;
                    offDiag -= scale*p[idx as usize + w as usize];
                }

                let newP = (b[idx as usize] - offDiag)/diag;

                p[idx as usize] = newP;
            }
        }
    }
}



pub fn relaxation(a: fn(i32, i32, i32) -> f64, x: &mut Vec<f64>, b: &Vec<f64>, w: i32, h: i32) {
    let limit = 100;

    let n = w*h;

    let mut temp = x.clone();


    for _ in 0..limit {
        for i in 0..n {
            let r = i / w;
            let c = i % w;

            // println!("{}", (r * w + c-1)  );
            // println!("{}", (r * w + c+1)   );
            // println!("{}", ((r+1) * w + c) );
            // println!("{}", ((r-1) * w + c)  );

            //     |p3|
            //  ---|--|---
            //  p1 |  | p2
            //  ---|--|---
            //     |p4|

            let p1 = if c - 1 >= 0 { x[ (r * w + c-1) as usize ] } else { 0.0 } * (0.01 / ( 1.0 * 1.0 * 1.0 ));
            let p2 = if c + 1 < w { x[ (r * w + c+1) as usize ] } else { 0.0 } * (0.01 / ( 1.0 * 1.0 * 1.0 ));
            let p3 = if r + 1 < h { x[ ((r+1) * w + c) as usize ] } else { 0.0 } * (0.01 / ( 1.0 * 1.0 * 1.0 ));
            let p4 = if r - 1 >= 0 { x[ ((r-1) * w + c) as usize ] } else { 0.0 } * (0.01 / ( 1.0 * 1.0 * 1.0 ));

            //println!("{} ", b[i as usize]);

            temp[i as usize] = ( b[i as usize] + p1 + p2 + p3 + p4 ) / (4.0 * (0.01 / ( 1.0 * 1.0 * 1.0 ))); 
        }
        *x = temp.clone();
    }
}


pub fn jacobi(a: fn(i32, i32, i32) -> f64, x: &mut Vec<f64>, b: &Vec<f64>, n: i32) {
    let limit = 500;

    let mut temp = x.clone();

    for _ in 0..limit {
        for i in 0..n {
            temp[i as usize] = 0.0;
            for j in 0..n {
                if i != j {
                    temp[i as usize] = temp[i as usize] + a(i, j, n) * x[j as usize];
                }
            }
            temp[i as usize] = ( b[i as usize] - temp[i as usize] ) / a(i, i, n);
        }
        *x = temp.clone();
    }
}



pub fn gauss_seidel(a: fn(i32, i32, i32) -> f64, x: &mut Vec<f64>, b: &Vec<f64>, n: i32) {
    let limit = 100;

    for _ in 0..limit {
        for i in 0..n {
            let mut sigma = 0.0;
            for j in 0..n {
                if i != j {
                    sigma = sigma + a(i, j, n) * x[j as usize];
                }
            }
            x[i as usize] = ( b[i as usize] - sigma ) / a(i, i, n);
        }
    }
}
