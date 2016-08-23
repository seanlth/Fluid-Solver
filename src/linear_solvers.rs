use std::mem;
use field::Field;
use scoped_threadpool::Pool;

use opencl;
use opencl::mem::CLBuffer;

use opencl_kernel::OpenCLKernel;


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

pub fn empty(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, opencl_kernel: Option<&OpenCLKernel>) {

}


pub fn relaxation(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, pencl_kernel: Option<&OpenCLKernel>) {
    let mut temp = x.clone();

    let w = x.columns;
    let h = x.rows;

    for _ in 0..limit {
        for r in 0..h {
            for c in 0..w {

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

                *temp.at_mut(r, c) = ( b.at(r, c) + p1 + p2 + p3 + p4 ) / (alpha * (dt / ( density * dx * dx )));
            }
        }
        *x = temp.clone();
    }
}

pub fn relaxation_unchecked(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize) {
    let mut temp = x.clone();

    let w = x.columns;
    let h = x.rows;

    let mut i = 0;
    while i < limit {
        let mut r = 0;
        while r < h {
            let mut c = 0;
            while c < w {

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

                *temp.at_fast_mut(r, c) = ( b.at_fast(r, c) + p1 + p2 + p3 + p4 ) / (alpha * (dt / ( density * dx * dx )));
                c += 1;
            }
            r += 1;
        }
        *x = temp.clone();
        i += 1;
    }
}

pub fn relaxation_fast(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize) {
    let columns = x.columns;
    let rows = x.rows;

    // add zero boundary
    let mut padded_x = vec![0.0; rows+2];
    for r in 0..rows {
        for c in 0..columns+2 {
            if c == 0 || c == columns+1 {
                padded_x.push(0.0);
            }
            else {
                padded_x.push(x.at_fast(r, c-1)); // offset
            }
        }
    }
    padded_x.append(&mut vec![0.0; rows+2]);

    let mut temp_field = Field::new(rows+2, columns+2, 0.0, 0.0);
    temp_field.field = padded_x;

    let mut temp = temp_field.clone();

    let scale = dt / ( density * dx * dx );

    let mut i = 0;
    while i < limit {
        let mut r = 1;
        while r < rows+1 {
            let mut c = 1;
            while c < columns+1 {
                let mut alpha = 4.0;

                alpha -= if c == 1 { 1.0 } else { 0.0 };
                alpha -= if c == columns { 1.0 } else { 0.0 };
                alpha -= if r == 1 { 1.0 } else { 0.0 };
                alpha -= if c == rows { 1.0 } else { 0.0 };

                let p1 = temp_field.at_fast(r, c-1);
                let p2 = temp_field.at_fast(r, c+1);
                let p3 = temp_field.at_fast(r-1, c);
                let p4 = temp_field.at_fast(r+1, c);

                let new = (  b.at_fast(r-1, c-1) + scale * ( p1 + p2 + p3 + p4 ) ) / (alpha * scale);
                *temp.at_fast_mut(r, c) = new;

                c += 1;
            }
            r += 1;
        }
        mem::swap(&mut temp.field, &mut temp_field.field);
        i += 1;
    }

    for r in 1..rows+1 {
        for c in 1..columns+1 {
            *x.at_fast_mut(r-1, c-1) = temp_field.at_fast(r, c);
        }
    }
}

#[link(name = "solver")]
extern {
    fn relaxation_ffi(x: *mut f64, x: *const f64, w: usize, h: usize, density: f64, dt: f64, dx: f64, limit: usize);
    fn relaxation_fast_ffi(x: *mut f64, x: *const f64, w: usize, h: usize, density: f64, dt: f64, dx: f64, limit: usize);
}

pub fn relaxation_c(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize) {
    let x_c = &mut x.field.as_mut_slice()[0] as *mut f64;
    let b_c = &b.field.as_slice()[0] as *const f64;

    unsafe {
        relaxation_ffi(x_c, b_c, x.columns, x.rows, density, dt, dx, limit);
    }
}

pub fn relaxation_fast_c(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, opencl_kernel: Option<&OpenCLKernel>) {
    let columns = x.columns;
    let rows = x.rows;

    let mut padded_x = vec![0.0; columns+2];
    for r in 0..rows {
        for c in 0..columns+2 {
            if c == 0 || c == columns+1 {
                padded_x.push(0.0);
            }
            else {
                padded_x.push(x.at(r, c-1)); // offset
            }
        }
    }
    padded_x.append(&mut vec![0.0; columns+2]);

    let x_c = &mut padded_x.as_mut_slice()[0] as *mut f64;
    let b_c = &b.field.as_slice()[0] as *const f64;

    unsafe {
        relaxation_fast_ffi(x_c, b_c, columns, rows, density, dt, dx, limit);
    }

    for r in 1..rows+1 {
        for c in 1..columns+1 {
            x.field[(r-1) * columns + (c-1) ] = padded_x[r * (columns+2) + c] as f64;
        }
    }
}

pub fn add_test(vec1: &Vec<f64>, vec2: &Vec<f64>, vec3: &mut Vec<f64>) {

    let mut i = 0;
    while i < 100000 {
        vec3[i] = vec1[i] + vec2[i];
        i += 1;
    }
}

pub fn add_opencl_test(vec1: &Vec<f64>, vec2: &Vec<f64>, vec3: &mut Vec<f64>)  {
    let group_size = 32;

    let ker = include_str!("kernels.cl");


    if let Ok((device, ctx, queue)) = opencl::util::create_compute_context_using_device(0) {

        //println!("{}", device.name());

        let a: CLBuffer<f64> = ctx.create_buffer(vec1.len(), opencl::cl::CL_MEM_READ_ONLY);
        let b: CLBuffer<f64> = ctx.create_buffer(vec1.len(), opencl::cl::CL_MEM_READ_ONLY);
        let c: CLBuffer<f64> = ctx.create_buffer(vec1.len(), opencl::cl::CL_MEM_WRITE_ONLY);

        queue.write(&a, &&vec1[..], ());
        queue.write(&b, &&vec2[..], ());

        let program = ctx.create_program_from_source(ker);
        program.build(&device).ok().expect("Couldn't build program.");

        let kernel = program.create_kernel("vector_add");

        kernel.set_arg(0, &a);
        kernel.set_arg(1, &b);
        kernel.set_arg(2, &c);

        let event = queue.enqueue_async_kernel(&kernel, vec1.len()-2, 1, Some(1), ());

        let vec_c: Vec<f64> = queue.get(&c, &event);
        *vec3 = vec_c.clone();

    }
}

pub fn relaxation_opencl(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, opencl_kernel: Option<&OpenCLKernel>) {
    let device = &opencl_kernel.unwrap().device;
    let ctx = &opencl_kernel.unwrap().ctx;
    let queue = &opencl_kernel.unwrap().queue;
    let kernel = &opencl_kernel.unwrap().kernel;

    let group_size_columns = 32;
    let group_size_rows = 32;

    let columns = x.columns;
    let rows = x.rows;

    // add zero boundary
    let mut padded_x = vec![0.0; columns+2];
    for r in 0..rows {
        for c in 0..columns+2 {
            if c == 0 || c == columns+1 {
                padded_x.push(0.0);
            }
            else {
                padded_x.push(x.at_fast(r, c-1)); // offset
            }
        }
    }
    padded_x.append(&mut vec![0.0; columns+2]);

    let mut new_x_buffer = &opencl_kernel.unwrap().buffers[0];
    let mut x_buffer = &opencl_kernel.unwrap().buffers[1];
    let b_buffer = &opencl_kernel.unwrap().buffers[2];

    let new_x_slice: &Vec<f32> = &padded_x.clone().iter().map(|v| *v as f32).collect();
    let x_slice: &Vec<f32> = &padded_x.clone().iter().map(|v| *v as f32).collect();
    let b_slice: &Vec<f32> = &b.field.clone().iter().map(|v| *v as f32).collect();

    queue.write(new_x_buffer, &&new_x_slice[..], ());
    queue.write(x_buffer, &&x_slice[..], ());
    queue.write(b_buffer, &&b_slice[..], ());


    kernel.set_arg(0, new_x_buffer);
    kernel.set_arg(1, x_buffer);
    kernel.set_arg(2, b_buffer);
    kernel.set_arg(3, &x.columns);
    kernel.set_arg(4, &x.rows);
    kernel.set_arg(5, &(density as f32));
    kernel.set_arg(6, &(dt as f32));
    kernel.set_arg(7, &(dx as f32));

    let mut event = queue.enqueue_async_kernel(&kernel, (x.columns, x.rows), (1, 1), Some((group_size_columns, group_size_rows)), ());

    mem::swap(&mut x_buffer, &mut new_x_buffer);

    let mut i = 0;
    while i < limit - 1 {

        kernel.set_arg(0, x_buffer);
        kernel.set_arg(1, new_x_buffer);
        kernel.set_arg(2, b_buffer);
        kernel.set_arg(3, &x.columns);
        kernel.set_arg(4, &x.rows);
        kernel.set_arg(5, &(density as f32));
        kernel.set_arg(6, &(dt as f32));
        kernel.set_arg(7, &(dx as f32));


        event = queue.enqueue_async_kernel(&kernel, (x.columns, x.rows), (1, 1), Some((group_size_columns, group_size_rows)), ());
        mem::swap(&mut x_buffer, &mut new_x_buffer);
        i += 1;
    }
    unsafe { opencl::cl::ll::clFlush(queue.cqueue) };
    unsafe { opencl::cl::ll::clFinish(queue.cqueue) };
    let result: Vec<f32> = if limit % 2 == 1 {
        queue.get(new_x_buffer, &event)
    }
    else {
        queue.get(x_buffer, &event)
    };

    for r in 1..rows+1 {
        for c in 1..columns+1 {
            x.field[(r-1) * columns + (c-1) ] = result[r * (columns+2) + c] as f64;
        }
    }

}


pub fn threaded_relaxation_unchecked(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize) {

    let temp = &mut x.clone();

    let x_clone = &x.clone();

    let w = x.columns;
    let h = x.rows;

    let threads = 8;
    let size = (w * h) / threads;

    let mut pool = Pool::new(threads as u32);

    for _ in 0..limit {

        let mut j = 0;
        let _ = pool.scoped(|scope| {
            for chunk in &mut temp.field.chunks_mut(size) {
                scope.execute(move || {
                    //let s = chunk.len();
                    for (k, chunk2) in chunk.chunks_mut(w).enumerate() {
                        let r = ( k + j*size ) / w;
                    for (c, e) in chunk2.iter_mut().enumerate() {
                        //let r = (c + j*size) / w;
                        // let c = (j + i*size) % w;

                        //     |p3|
                        //  ---|--|---
                        //  p1 |  | p2
                        //  ---|--|---
                        //     |p4|


                        let mut alpha = 4.0;

                        let p1 = if c as i32 - 1 >= 0 { x_clone.at_fast(r, c-1) } else { alpha -= 1.0; 0.0 } * (dt / ( density * dx * dx ));
                        let p2 = if c as i32 + 1 < w as i32 { x_clone.at_fast(r, c+1) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                        let p3 = if r as i32 + 1 < h as i32 { x_clone.at_fast(r+1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));
                        let p4 = if r as i32 - 1 >= 0 { x_clone.at_fast(r-1, c) } else { alpha-=1.0; 0.0 } * (dt / ( density * dx * dx ));

                        *e = ( b.at_fast(r, c) + p1 + p2 + p3 + p4 ) / (alpha * (dt / ( density * dx * dx )));
                    }
                    }
                });
            }
            j += 1;
        });
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
