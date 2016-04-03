
use interpolation;
use integrators;
use field::Field;

// template advection
pub fn empty_advection(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64, integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {

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
pub fn semi_lagrangian(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64, interpolator: &Fn(f64, f64, &Field) -> f64, integrator: &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) {
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
// pub fn semi_lagrangian_opencl(field: &mut Field, u: &Field, v: &Field, dt: f64, dx: f64) {
//     let group_size_columns = 32;
//     let group_size_rows = 32;
//
//     let columns = x.columns;
//     let rows = x.rows;
//
//     let ker = include_str!("kernels.cl");
//
//     if let Ok((device, ctx, queue)) = opencl::util::create_compute_context_using_device(2) {
//
//         let mut field_buffer: CLBuffer<f32> = ctx.create_buffer(padded_x.len(), opencl::cl::CL_MEM_READ_WRITE);
//         let mut u_buffer: CLBuffer<f32> = ctx.create_buffer(padded_x.len(), opencl::cl::CL_MEM_READ_WRITE);
//         let b_buffer: CLBuffer<f32> = ctx.create_buffer(b.field.len(), opencl::cl::CL_MEM_READ_ONLY);
//
//         let new_x_slice: &Vec<f32> = &padded_x.clone().iter().map(|v| *v as f32).collect();
//         let x_slice: &Vec<f32> = &padded_x.clone().iter().map(|v| *v as f32).collect();
//         let b_slice: &Vec<f32> = &b.field.clone().iter().map(|v| *v as f32).collect();
//
//         queue.write(&new_x_buffer, &&new_x_slice[..], ());
//         queue.write(&x_buffer, &&x_slice[..], ());
//         queue.write(&b_buffer, &&b_slice[..], ());
//
//         let program = ctx.create_program_from_source(ker);
//         program.build(&device).ok().expect("Couldn't build program.");
//
//         let kernel = program.create_kernel("relaxation");
//
//         let s: f32 = 1.0;
//         //kernel.set_local(0, x.field.len(), &s);
//         //kernel.set_local(1, group_size_columns * group_size_rows, &s);
//         kernel.set_arg(0, &new_x_buffer);
//         kernel.set_arg(1, &x_buffer);
//         kernel.set_arg(2, &b_buffer);
//         kernel.set_arg(3, &x.columns);
//         kernel.set_arg(4, &x.rows);
//         kernel.set_arg(5, &(density as f32));
//         kernel.set_arg(6, &(dt as f32));
//         kernel.set_arg(7, &(dx as f32));
//
//         let mut event = queue.enqueue_async_kernel(&kernel, (x.columns, x.rows), (1, 1), Some((group_size_columns, group_size_rows)), ());
//
//         mem::swap(&mut x_buffer, &mut new_x_buffer);
//
//         let mut i = 0;
//         while i < limit - 1 {
//             //if i % 2 == 0 {
//                 //kernel.set_local(0, x.field.len(), &s);
//                 //kernel.set_local(1, group_size_columns * group_size_rows, &s);
//                 kernel.set_arg(0, &x_buffer);
//                 kernel.set_arg(1, &new_x_buffer);
//                 kernel.set_arg(2, &b_buffer);
//                 kernel.set_arg(3, &x.columns);
//                 kernel.set_arg(4, &x.rows);
//                 kernel.set_arg(5, &(density as f32));
//                 kernel.set_arg(6, &(dt as f32));
//                 kernel.set_arg(7, &(dx as f32));
//             //}
//             // else {
//             //     //kernel.set_local(0, x.field.len(), &s);
//             //     //kernel.set_local(1, group_size_columns * group_size_rows, &s);
//             //     kernel.set_arg(0, &new_x_buffer);
//             //     kernel.set_arg(1, &x_buffer);
//             //     kernel.set_arg(2, &b_buffer);
//             //     kernel.set_arg(3, &x.columns);
//             //     kernel.set_arg(4, &x.rows);
//             //     kernel.set_arg(5, &(density as f32));
//             //     kernel.set_arg(6, &(dt as f32));
//             //     kernel.set_arg(7, &(dx as f32));
//             // }
//
//             event = queue.enqueue_async_kernel(&kernel, (x.columns, x.rows), (1, 1), Some((group_size_columns, group_size_rows)), ());
//             mem::swap(&mut x_buffer, &mut new_x_buffer);
//             i += 1;
//         }
//
//         unsafe { opencl::cl::ll::clFinish(queue.cqueue) };
//         let result: Vec<f32> = if limit % 2 == 1 {
//             queue.get(&new_x_buffer, &event)
//         }
//         else {
//             queue.get(&x_buffer, &event)
//         };
//
//         //let result: Vec<f32> = queue.get(&new_x_buffer, &event);
//         //println!("{:?}", result);
//         //x.field = result.clone().iter().map(|v| *v as f64).collect();
//         for r in 1..rows+1 {
//             for c in 1..columns+1 {
//                 x.field[(r-1) * columns + (c-1) ] = result[r * (columns+2) + c] as f64;
//             }
//         }
//     }
//     //}
//
// 	let c = field.columns;
// 	let r = field.rows;
//
// 	let mut temp = field.clone();
//
//
// 	for j in 0..r {
// 		for i in 0..c {
//             // position on staggered grid, field_array(i, j) -> grid(x, y)
//             let x = i as f64 + field.offset_x;
//             let y = j as f64 + field.offset_y;
//
//             let f1 = |_: f64, _: f64| -interpolator(x, y, &u)/dx;
//             let f2 = |_: f64, _: f64| -interpolator(x, y, &v)/dx;
//
//             let old_x = integrator(x, 0.0, &f1, dt);
//             let old_y = integrator(y, 0.0, &f2, dt);
//
//             // translate grid(old_x, old_y) -> field_array(i, j)
// 			*temp.at_fast_mut(j, i) = interpolator(old_x, old_y, field);
// 		}
// 	}
//
// 	*field = temp;
// }
