
use field::Field;

//      0        1        2
//   0  1  2  0  1  2  0  1  2
//   0  1  2  3  4  5  6  7  8
//
//1  4 -1  0 -1  0  0  0  0  0
//2 -1  4 -1  0 -1  0  0  0  0
//3  0 -1  4  0  0 -1  0  0  0
//1 -1  0  0  4 -1  0 -1  0  0
//2  0 -1  0 -1  4 -1  0 -1  0
//3  0  0 -1  0 -1  4  0  0 -1
//1  0  0  0 -1  0  0  4 -1  0
//2  0  0  0  0 -1  0 -1  4 -1
//3  0  0  0  0  0 -1  0 -1  4

pub fn laplacian(r: i32, c: i32, n: i32) -> f64 {
    let c_x = r % n;
    let c_y = r / n;

    let x = c % n;
    let y = c / n;

    if c_x == x && c_y == y { 4.0 }
    else if (c_x - x).abs() + (c_y - y).abs() == 1 { -1.0 }
    else { 0.0 }
}

pub fn fast_access_mut<T>(arr: &mut Vec<T>, idx: i32) -> &mut T {
    unsafe {
        arr.get_unchecked_mut(idx as usize)
    }
}

pub fn fast_access<T>(arr: &Vec<T>, idx: i32) -> &T {
    unsafe {
        arr.get_unchecked(idx as usize)
    }
}

pub fn relaxation(x: &mut Field, b: &Field, w: usize, h: usize, density: f64, dt: f64, dx: f64, limit: usize) {
    let mut temp = x.clone();

    for _ in 0..limit {
        for c in 0..h {
            for r in 0..w {

                //     |p3|
                //  ---|--|---
                //  p1 |  | p2
                //  ---|--|---
                //     |p4|


                let mut alpha = 4.0;


                let p1 = if c as i32 - 1 >= 0 { x.at(r, c-1) } else { alpha -= 1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p2 = if c as i32 + 1 < w as i32 { x.at(r, c+1) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p3 = if r as i32 + 1 < h as i32 { x.at(r+1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p4 = if r as i32 - 1 >= 0 { x.at(r-1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));

                *temp.at_mut(r, c) = b.at(r, c) + p1 + p2 + p3 + p4 / (alpha * (dt / ( density * dx * dx )));
            }
        }
        *x = temp.clone();
    }
}

pub fn relaxation_unchecked(x: &mut Field, b: &Field, w: usize, h: usize, density: f64, dt: f64, dx: f64, limit: usize) {
    let mut temp = x.clone();

    for _ in 0..limit {
        for c in 0..h {
            for r in 0..w {

                //     |p3|
                //  ---|--|---
                //  p1 |  | p2
                //  ---|--|---
                //     |p4|


                let mut alpha = 4.0;


                let p1 = if c as i32 - 1 >= 0 { x.at_fast(r, c-1) } else { alpha -= 1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p2 = if c as i32 + 1 < w as i32 { x.at_fast(r, c+1) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p3 = if r as i32 + 1 < h as i32 { x.at_fast(r+1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                let p4 = if r as i32 - 1 >= 0 { x.at_fast(r-1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));

                *temp.at_fast_mut(r, c) = b.at_fast(r, c) + p1 + p2 + p3 + p4 / (alpha * (dt / ( density * dx * dx )));
            }
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
