
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

pub fn relaxation(x: &mut Vec<f64>, b: &Vec<f64>, w: i32, h: i32, density: f64, dt: f64, dx: f64, limit: usize) {
    let n = w*h;

    let mut temp = x.clone();


    for _ in 0..limit {
        for i in 0..n {
            let r = i / w;
            let c = i % w;

            //     |p3|
            //  ---|--|---
            //  p1 |  | p2
            //  ---|--|---
            //     |p4|


            let p1 = if c - 1 >= 0 { x[ (r * w + c-1) as usize ] } else { 0.0 } * (dt / ( density * dx * dx ));
            let p2 = if c + 1 < w { x[ (r * w + c+1) as usize ] } else { 0.0 } * (dt / ( density * dx * dx ));
            let p3 = if r + 1 < h { x[ ((r+1) * w + c) as usize ] } else { 0.0 } * (dt / ( density * dx * dx ));
            let p4 = if r - 1 >= 0 { x[ ((r-1) * w + c) as usize ] } else { 0.0 } * (dt / ( density * dx * dx ));


            temp[i as usize] = ( *fast_access(b, i) + p1 + p2 + p3 + p4 ) / (4.0 * (dt / ( density * dx * dx )));
        }
        *x = temp.clone();
    }
}


pub fn relaxation_unchecked(x: &mut Vec<f64>, b: &Vec<f64>, w: i32, h: i32, density: f64, dt: f64, dx: f64, limit: usize) {
    let n = w*h;

    let mut temp = x.clone();


    for _ in 0..limit {
        for i in 0..n {
            let r = i / w;
            let c = i % w;

            //     |p3|
            //  ---|--|---
            //  p1 |  | p2
            //  ---|--|---
            //     |p4|


            let mut alpha = 4.0;

            let p1 = if c - 1 >= 0 { *fast_access(x, r * w + c-1) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
            let p2 = if c + 1 < w { *fast_access(x, r * w + c+1) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
            let p3 = if r + 1 < h { *fast_access(x, (r+1) * w + c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
            let p4 = if r - 1 >= 0 { *fast_access(x, (r-1) * w + c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));


            *fast_access_mut(&mut temp, i) = ( *fast_access(b, i) + p1 + p2 + p3 + p4 ) / (alpha * (dt / ( density * dx * dx )));
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
