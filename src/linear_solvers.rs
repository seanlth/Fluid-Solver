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



pub fn jacobi(a: fn(i32, i32, i32) -> f64, x: &mut Vec<f64>, b: &Vec<f64>, n: i32) {
    let limit = 100;

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
