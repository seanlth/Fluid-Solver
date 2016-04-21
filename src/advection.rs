
use interpolation;
use integrators;
use field::Field;
use opencl;
use opencl::mem::CLBuffer;
use opencl::hl::*;

use opencl_kernel::OpenCLKernel;

// pub fn setup_advection_kernel() -> Result<(Device, Context, CommandQueue, Kernel), String> {
//     let ker = include_str!("kernels.cl");
//
//     if let Ok((device, ctx, queue)) = opencl::util::create_compute_context_using_device(2) {
//
//         let program = ctx.create_program_from_source(ker);
//         let info = program.build(&device);
//         if let Result::Err(s) = info {
//             println!("{}", s);
//             return Err(s.to_string());
//         }
//
//         let kernel = program.create_kernel("semi_lagrangian");
//         return Ok((device, ctx, queue, kernel))
//     }
//     Err("Error".to_string())
// }


// template advection
pub fn empty_advection(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64, integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64, kernel: Option<&OpenCLKernel>) {

}

// upwind with bicubic interpolation
pub fn upwind_advection(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64, integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {
	let c = field.columns;
	let r = field.rows;

	let mut temp = field.clone();

	for i in 0..r {
		for j in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = j as f64 + field.offset_x;
            let y = i as f64 + field.offset_y;

			let u_velocity = interpolator(x, y, u) / dx;
			let v_velocity = interpolator(x, y, v) / dx;

			let u_velocity_sign = u_velocity.signum();
			let v_velocity_sign = v_velocity.signum();

			let a = if j > 0 && j < c - 1 { (j as f64 - u_velocity_sign) as usize } else { j };
			let b = if i > 0 && i < r - 1 { (i as f64 - v_velocity_sign) as usize } else { i };

			*temp.at_mut(i, j) = field.at(i, j) - (dt / dx) * u_velocity * ( u_velocity_sign * field.at(i, j) - u_velocity_sign * field.at(i, a) )
									 - (dt / dx) * v_velocity * ( v_velocity_sign * field.at(i, j) - v_velocity_sign * field.at(b, j) );

		}
	}

	*field = temp;
}

// semi-lagrangian backtrace
pub fn semi_lagrangian(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64, integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64, _: Option<&OpenCLKernel>) {
	let c = field.columns;
	let r = field.rows;

	let mut temp = field.clone();

	for j in 0..r {
		for i in 0..c {
            // position on staggered grid, field_array(i, j) -> grid(x, y)
            let x = i as f64 + field.offset_x;
            let y = j as f64 + field.offset_y;

            let f1 = |o: f64, _: f64| -interpolator(o, y, &u)/dx;
            let f2 = |o: f64, _: f64| -interpolator(x, o, &v)/dx;

            let old_x = integrator(x, 0.0, &f1, dt);
            let old_y = integrator(y, 0.0, &f2, dt);

            // translate grid(old_x, old_y) -> field_array(i, j)
			*temp.at_fast_mut(j, i) = interpolator(old_x, old_y, field);
		}
	}

	*field = temp;
}

// semi-lagrangian backtrace
pub fn semi_lagrangian_opencl(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, _: &Fn(f64, f64, &Field) -> f64, _: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64, opencl_kernel: Option<&OpenCLKernel>) {
    let device = &opencl_kernel.unwrap().device;
    let ctx = &opencl_kernel.unwrap().ctx;
    let queue = &opencl_kernel.unwrap().queue;
    let kernel = &opencl_kernel.unwrap().kernel;


    let group_size_columns = 32;
    let group_size_rows = 32;

    let rows = field.rows;
    let columns = field.columns;

    let padded_columns = columns + 32 - (((columns-1) % 32) + 1);
    let padded_rows = rows + 32 - (((rows-1) % 32) + 1);

    let mut padded_u_columns = padded_columns;
    let mut padded_u_rows = padded_rows;

    let mut padded_v_columns = padded_columns;
    let mut padded_v_rows = padded_rows;

    let mut padded_field = vec![];
    let mut padded_u = vec![];
    let mut padded_v = vec![];

    for r in 0..padded_rows {
        for c in 0..padded_columns {
            if c >= columns || r >= rows {
                padded_field.push(0.0);
                padded_v.push( if r == rows { v.at_fast(r, c) } else { 0.0 } );
                padded_u.push( if c == columns { u.at_fast(r, c) } else { 0.0 } );
            }
            else {
                padded_field.push(field.at_fast(r, c));
                padded_u.push(u.at_fast(r, c));
                padded_v.push(v.at_fast(r, c));
            }
        }
    }

    if padded_rows == rows && padded_columns == columns {
        padded_u = u.field.clone();
        padded_v = v.field.clone();

        padded_u_columns = u.columns;
        padded_u_rows = u.rows;
        padded_v_columns = v.columns;
        padded_v_rows = v.rows;
    }



    // columns = columns + remainder_columns;
    // rows = rows + remainder_rows;

    //}

    // if rows % 32 != 0 {
    //     let remainder = 32 - rows % 32;
    //     padded_field.append(&mut vec![0.0; remainder * columns]);
    //     // for r in 0..rows+remainder {
    //     //     for c in 0..field.columns {
    //     //         if r >= rows {
    //     //             padded_field.push(0.0);
    //     //         }
    //     //         else {
    //     //             padded_field.push(field.at_fast(r, c)); // offset
    //     //         }
    //     //     }
    //     // }
    //     rows = rows + remainder;
    // }
    //
    // if field.rows % 32 == 0 && field.columns % 32 == 0 {
    //     padded_field = field.field.clone();
    // }

    // println!("old rows {:?}", rows);
    // println!("old columns {:?}", columns);
    //
    // println!("new rows {:?}", padded_rows);
    // println!("new columns {:?}", padded_columns);

    //println!("{:?}", padded_field.len());

    //let ker = include_str!("kernels.cl");

    //if let Ok((device, ctx, queue)) = opencl::util::create_compute_context_using_device(2) {

        // let field_buffer: CLBuffer<f32> = ctx.create_buffer(padded_field.len(), opencl::cl::CL_MEM_READ_ONLY);
        // let temp_buffer: CLBuffer<f32> = ctx.create_buffer(padded_field.len(), opencl::cl::CL_MEM_READ_WRITE);
        // let u_buffer: CLBuffer<f32> = ctx.create_buffer(padded_u.len(), opencl::cl::CL_MEM_READ_ONLY);
        // let v_buffer: CLBuffer<f32> = ctx.create_buffer(padded_v.len(), opencl::cl::CL_MEM_READ_ONLY);

        // let field_buffer1 = &opencl_kernel.unwrap().buffers[0];
        // let temp_buffer1 = &opencl_kernel.unwrap().buffers[1];
        // let field_buffer2 = &opencl_kernel.unwrap().buffers[2];
        // let temp_buffer2 = &opencl_kernel.unwrap().buffers[3];
        // let field_buffer3 = &opencl_kernel.unwrap().buffers[4];
        // let temp_buffer3 = &opencl_kernel.unwrap().buffers[5];
        //
        // let u_buffer = &opencl_kernel.unwrap().buffers[6];
        // let v_buffer = &opencl_kernel.unwrap().buffers[7];

        let field_slice: &Vec<f32> = &padded_field.clone().iter().map(|v| *v as f32).collect();
        let temp_slice: &Vec<f32> = &padded_field.clone().iter().map(|v| *v as f32).collect();
        let u_slice: &Vec<f32> = &padded_u.clone().iter().map(|v| *v as f32).collect();
        let v_slice: &Vec<f32> = &padded_v.clone().iter().map(|v| *v as f32).collect();


        let field_buffer: &CLBuffer<f32>;
        let temp_buffer: &CLBuffer<f32>;
        let u_buffer: &CLBuffer<f32>;
        let v_buffer: &CLBuffer<f32>;

        // invoked with u
        if rows == u.rows && columns == u.columns {
            field_buffer = &opencl_kernel.unwrap().buffers[4];
            temp_buffer = &opencl_kernel.unwrap().buffers[5];
            u_buffer = &opencl_kernel.unwrap().buffers[6];
            v_buffer = &opencl_kernel.unwrap().buffers[7];
        }
        else if rows == v.rows && columns == v.columns { // invoked with v
            
            field_buffer = &opencl_kernel.unwrap().buffers[8];
            temp_buffer = &opencl_kernel.unwrap().buffers[9];
            u_buffer = &opencl_kernel.unwrap().buffers[10];
            v_buffer = &opencl_kernel.unwrap().buffers[11];
        }
        else { // invoked with field
            field_buffer = &opencl_kernel.unwrap().buffers[0];
            temp_buffer = &opencl_kernel.unwrap().buffers[1];
            u_buffer = &opencl_kernel.unwrap().buffers[2];
            v_buffer = &opencl_kernel.unwrap().buffers[3];
        }

        queue.write(field_buffer, &&field_slice[..], ());
        queue.write(temp_buffer, &&temp_slice[..], ());
        queue.write(u_buffer, &&u_slice[..], ());
        queue.write(v_buffer, &&v_slice[..], ());

        // let program = ctx.create_program_from_source(ker);
        // let info = program.build(&device);
        // if let Result::Err(s) = info {
        //     println!("{}", s);
        //     panic!()
        // }
        //
        // let kernel = program.create_kernel("semi_lagrangian");

        kernel.set_arg(0, field_buffer);
        kernel.set_arg(1, temp_buffer);
        kernel.set_arg(2, u_buffer);
        kernel.set_arg(3, v_buffer);
        kernel.set_arg(4, &(dt as f32));
        kernel.set_arg(5, &(dx as f32));
        kernel.set_arg(6, &(field.offset_x as f32));
        kernel.set_arg(7, &(field.offset_y as f32));
        kernel.set_arg(8, &padded_rows);
        kernel.set_arg(9, &padded_columns);
        kernel.set_arg(10, &(u.offset_x as f32));
        kernel.set_arg(11, &(u.offset_y as f32));
        kernel.set_arg(12, &padded_u_rows);
        kernel.set_arg(13, &padded_u_columns);
        kernel.set_arg(14, &(v.offset_x as f32));
        kernel.set_arg(15, &(v.offset_y as f32));
        kernel.set_arg(16, &padded_v_rows);
        kernel.set_arg(17, &padded_v_columns);

        let event = queue.enqueue_async_kernel(&kernel, (padded_rows, padded_rows), (0, 0), Some((group_size_columns, group_size_rows)), ());

        unsafe { opencl::cl::ll::clFinish(queue.cqueue) };
        let result: Vec<f32> = queue.get(temp_buffer, &event);

        //if columns != padded_columns {
            //println!("columns");
            for r in 0..rows {
                for c in 0..columns {
                    *field.at_mut(r, c) = result[r * (padded_columns) + c] as f64;
                }
            }
        // }
        // else if rows != padded_rows {
        //     //println!("rows");
        //     for r in 0..rows {
        //         for c in 0..columns {
        //             *field.at_mut(r, c) = result[r * (columns) + c] as f64;
        //         }
        //     }
        // }
        // else {
        //     //println!("none");
        //     field.field = result.into_iter().map(|v| v as f64).collect();
        // }
    //}
}
