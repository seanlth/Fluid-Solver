extern crate lodepng;

use std::mem::swap;
use std::cmp;
use std::io::prelude::*;
use std::fs::File;

use field::Field;
use linear_solvers;
use integrators;
use advection;
use interpolation;

use opencl_kernel::OpenCLKernel;
use opencl;

pub struct FluidSolver {
	pub velocity_x: Field,
	pub velocity_y: Field,
	pub density: Field,
	pub pressure: Field,
	pub divergence: Field,
	pub particles: Vec<(f64, f64)>,
	pub rows: usize,
	pub columns: usize,
	dt: f64,
	dx: f64,
	fluid_density: f64,
    gravity: f64,
    advection: fn(&mut Field, &Field, &Field, f64, f64, &Fn(f64, f64, &Field) -> f64, &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64, kernel: Option<&OpenCLKernel>),
    interpolation: fn(mut x: f64, mut y: f64, field: &Field) -> f64,
    integration: fn(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64,
    linear_solver: fn(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, kernel: Option<&OpenCLKernel>),
    advect_kernel: OpenCLKernel,
    linear_solver_kernel: OpenCLKernel
}

impl FluidSolver {
	pub fn new(fluid_density: f64, rows: usize, columns: usize, dt: f64, dx: f64, gravity: f64) -> FluidSolver  {

 		let mut f = FluidSolver {
 			velocity_x: Field::new(rows, columns+1, 0.0, 0.5),
 			velocity_y: Field::new(rows+1, columns, 0.5, 0.0),
			density: Field::new(rows, columns, 0.5, 0.5),
			pressure: Field::new(rows, columns, 0.5, 0.5),
			divergence: Field::new(rows, columns, 0.5, 0.5),
            particles: Vec::new(),
 			rows: rows,
 			columns: columns,
			dt: dt,
			dx: dx,
            fluid_density: fluid_density,
            gravity: gravity,
            advection: advection::empty_advection,
            interpolation: interpolation::empty_interpolate,
            integration: integrators::empty,
            linear_solver: linear_solvers::empty,
            advect_kernel: OpenCLKernel::new("semi_lagrangian").unwrap(),
            linear_solver_kernel: OpenCLKernel::new("relaxation").unwrap()
 		};

        // field buffer
        let padded_field_columns = columns + 32 - (((columns-1) % 32) + 1);
        let padded_field_rows = rows + 32 - (((rows-1) % 32) + 1);

        // no padding needed
        if padded_field_columns == columns && padded_field_rows == rows {
            f.advect_kernel.create_buffer(columns*rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(columns*rows, opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(f.velocity_x.field.len(), opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(f.velocity_y.field.len(), opencl::cl::CL_MEM_READ_ONLY);
        }
        else {
            f.advect_kernel.create_buffer(padded_field_columns*padded_field_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_field_columns*padded_field_rows, opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(padded_field_columns*padded_field_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_field_columns*padded_field_rows, opencl::cl::CL_MEM_READ_ONLY);
        }

        // u buffer
        let padded_u_columns = (columns+1) + 32 - ((((columns+1)-1) % 32) + 1);
        let padded_u_rows = (rows) + 32 - ((((rows)-1) % 32) + 1);

        // no padding needed
        if padded_u_columns == f.velocity_x.columns && padded_u_rows == f.velocity_x.rows {
            f.advect_kernel.create_buffer(f.velocity_x.field.len(), opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(f.velocity_x.field.len(), opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(f.velocity_x.field.len(), opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(f.velocity_y.field.len(), opencl::cl::CL_MEM_READ_ONLY);
        }
        else {
            f.advect_kernel.create_buffer(padded_u_columns*padded_u_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_u_columns*padded_u_rows, opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(padded_u_columns*padded_u_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_u_columns*padded_u_rows, opencl::cl::CL_MEM_READ_ONLY);
        }

        // v buffer
        let padded_v_columns = (columns) + 32 - ((((columns)-1) % 32) + 1);
        let padded_v_rows = (rows+1) + 32 - ((((rows+1)-1) % 32) + 1);

        // no padding needed
        if padded_v_columns == f.velocity_y.columns && padded_v_rows == f.velocity_y.rows {
            f.advect_kernel.create_buffer(f.velocity_y.field.len(), opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(f.velocity_y.field.len(), opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(f.velocity_x.field.len(), opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(f.velocity_y.field.len(), opencl::cl::CL_MEM_READ_ONLY);
        }
        else {
            //println!("init {}", padded_v_columns*padded_v_rows);
            // println!("init {}", f.velocity_x.field.len());
            // println!("init {}", f.velocity_y.field.len());

            f.advect_kernel.create_buffer(padded_v_columns*padded_v_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_v_columns*padded_v_rows, opencl::cl::CL_MEM_READ_WRITE);
            f.advect_kernel.create_buffer(padded_v_columns*padded_v_rows, opencl::cl::CL_MEM_READ_ONLY);
            f.advect_kernel.create_buffer(padded_v_columns*padded_v_rows, opencl::cl::CL_MEM_READ_ONLY);
        }


        //
        // let s1 = padded_columns1 * padded_rows1;
        // let s2 = padded_columns2 * padded_rows1; // u
        // let s3 = padded_columns1 * padded_rows2; // v
        //
        // //println!("{}", s1 );
        // f.advect_kernel.create_buffer(s1, opencl::cl::CL_MEM_READ_ONLY);
        // f.advect_kernel.create_buffer(s1, opencl::cl::CL_MEM_READ_WRITE);
        // f.advect_kernel.create_buffer(s2, opencl::cl::CL_MEM_READ_ONLY);
        // f.advect_kernel.create_buffer(s2, opencl::cl::CL_MEM_READ_WRITE);
        // f.advect_kernel.create_buffer(s3, opencl::cl::CL_MEM_READ_ONLY);
        // f.advect_kernel.create_buffer(s3, opencl::cl::CL_MEM_READ_WRITE);


        f.linear_solver_kernel.create_buffer((columns+2)*(rows*2), opencl::cl::CL_MEM_READ_WRITE);
        f.linear_solver_kernel.create_buffer((columns+2)*(rows*2), opencl::cl::CL_MEM_READ_WRITE);
        f.linear_solver_kernel.create_buffer((columns)*(rows), opencl::cl::CL_MEM_READ_WRITE);

        f
 	}

    pub fn use_markers(mut self) -> Self {
		for i in 0..self.rows*2 {
			for j in 0..self.columns*2 {
				self.particles.push( (j as f64 * self.dx / 2.0, i as f64*self.dx / 2.0 ) );
			}
		}
        self
    }

    pub fn advection(mut self, f: fn(&mut Field, &Field, &Field, f64, f64, &Fn(f64, f64, &Field) -> f64, &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64, Option<&OpenCLKernel>) ) -> Self {
        self.advection = f;
        self
    }

    pub fn interpolation(mut self, f: fn(mut x: f64, mut y: f64, field: &Field) -> f64) -> Self {
        self.interpolation = f;
        self
    }

    pub fn integration(mut self, f: fn(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64) -> Self {
        self.integration = f;
        self
    }

    pub fn linear_solver(mut self, f: fn(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize, kernel: Option<&OpenCLKernel>) ) -> Self {
        self.linear_solver = f;
        self
    }

	// LP = D
	// see diagram
	pub fn pressure_solve(&mut self) {
		(self.linear_solver)( &mut self.pressure, &self.divergence, self.fluid_density, self.dt, self.dx, 600, Some(&self.linear_solver_kernel) );
        //linear_solvers::relaxation_fast_c( &mut self.pressure, &self.divergence, self.fluid_density, self.dt, self.dx, 600 );


	}

	pub fn solve(&mut self) {
		self.apply_gravity();
		self.project();
		self.advect();
	}

	pub fn apply_gravity(&mut self) {
		for r in 0..self.rows+1 {
			for c in 0..self.columns {
				*self.velocity_y.at_mut(r, c) -= self.gravity * self.dt;
			}
		}
	}

	pub fn advect_particles(&mut self) {

        let u = self.velocity_x.clone();
        let v = self.velocity_y.clone();

		for p in &mut self.particles {
			let (x, y) = *p;

            let f1 = |o: f64, t: f64| interpolation::bicubic_interpolate(o, y, &u);
            let f2 = |o: f64, t: f64| interpolation::bicubic_interpolate(x, o, &v);

			// let u = interpolation::bicubic_interpolate(x, y, &self.velocity_x);
			// let v = interpolation::bicubic_interpolate(x, y, &self.velocity_y);

            let x = (self.integration)(x, 0.0, &f1, self.dt);
            let y = (self.integration)(y, 0.0, &f2, self.dt);
            *p = (x, y);

			//*p = integrators::euler(x, y, u, v, self.dt);
            //*p = ( integrators::euler(x, 0.0, f1, self.dt), integrators::euler(y, 0.0, f2, self.dt) )

		}
	}

	pub fn advect(&mut self) {

		let u = self.velocity_x.clone();
		let v = self.velocity_y.clone();

        // println!("{}", self.velocity_x.columns);
        // println!("{}", self.velocity_x.rows);

		(self.advection)(&mut self.velocity_x, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration, Some(&self.advect_kernel));
		(self.advection)(&mut self.velocity_y, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration, Some(&self.advect_kernel));
		(self.advection)(&mut self.density, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration, Some(&self.advect_kernel));

        // advection::semi_lagrangian_opencl(&mut self.velocity_x, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration, &self.advect_kernel);
		// advection::semi_lagrangian_opencl(&mut self.velocity_y, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration,  &self.advect_kernel);
		// advection::semi_lagrangian_opencl(&mut self.density, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration,  &self.advect_kernel);

		self.advect_particles();
	}

	pub fn calculate_divergence(&mut self) {
		for i in 0..self.rows {
			for j in 0..self.columns {

				let x_velocity1 = if j < self.columns-1 { self.velocity_x.at(i, j+1) } else { 0.0 };
				let x_velocity2 = if j > 0 { self.velocity_x.at(i, j) } else { 0.0 };

				let y_velocity1 = if i < self.rows-1 { self.velocity_y.at(i + 1, j) } else { 0.0 };
				let y_velocity2 = if i > 0 { self.velocity_y.at(i, j) } else { 0.0 };

				*self.divergence.at_mut(i, j) = -(x_velocity1 - x_velocity2 + y_velocity1 - y_velocity2) / self.dx;
			}
		}
	}

	pub fn set_boundaries(&mut self) {
		for i in 0..self.rows {
			*self.velocity_x.at_mut(i, 0) = 0.0;
			*self.velocity_x.at_mut(i, self.columns) = 0.0;
		}
		for i in 0..self.columns {
			*self.velocity_y.at_mut(0, i) = 0.0;
			*self.velocity_y.at_mut(self.rows, i) = 0.0;
		}
	}

	pub fn project(&mut self) {
		self.calculate_divergence();
		self.pressure_solve();

		// apply pressure to velocity
		for i in 0..self.rows+1 {
			for j in 0..self.columns+1 {
				//let p = i * self.columns + j;

				let p1 = if j < self.columns && i < self.rows { self.pressure.at(i, j) } else { 0.0 };
				let p2 = if i < self.rows && j < self.columns { self.pressure.at(i, j) } else { 0.0 };

				let p3 = if j as i32 - 1 >= 0 && i < self.rows { self.pressure.at(i, j - 1) } else { 0.0 };
				let p4 = if i as i32 - 1 >= 0 && j < self.columns { self.pressure.at(i - 1, j) } else { 0.0 };


				if j <= self.columns && i < self.rows {
					*self.velocity_x.at_mut(i, j) = self.velocity_x.at(i, j) - (self.dt / (self.fluid_density * self.dx)) * ( p1 - p3 );
				}
				if j < self.columns && i <= self.rows {
					*self.velocity_y.at_mut(i, j) = self.velocity_y.at(i, j) - (self.dt / (self.fluid_density * self.dx)) * ( p2 - p4 );
				}
			}
		}

		// boundary conditions
		self.set_boundaries();
	}

    // pub fn add_source(&mut self, r: usize, c: usize, w: usize, h: usize, velocity_x: f64, velocty_y: f64, density: f64) {
    //     let start_r = cmp::max(r, 0);
    //     let end_r = cmp::min(self.rows, r+h);
    //
    //     let start_c = cmp::max(c, 0);
    //     let end_c = cmp::min(self.columns, c+w);
    //
    //     for i in start_r..end_r {
    //         for j in start_c..end_c {
    //             *self.velocity_x.at_mut(i, j) = velocity_x;
    //             *self.velocity_y.at_mut(i, j) = velocty_y;
    //             *self.density.at_mut(i, c) = density;
    //         }
    //     }
    // }

	pub fn add_source(&mut self, r: usize, c: usize, velocity_x: f64, velocity_y: f64, density: f64) {
		*self.velocity_x.at_mut(r, c) = velocity_x;
        //*self.velocity_y.at_mut(r, c) = velocity_y;
        *self.density.at_mut(r, c) = density;

		//*self.velocity_x.at_mut(r, c) = velocity_x;
		//*self.velocity_y.at_mut(r + 1, c) = velocty_y;
		//*self.velocity_y.at_mut(r, c) = velocty_y;
		// *self.density.at_mut(r, c) = density;
        // *self.density.at_mut(r, c + 1) = density;
        // *self.density.at_mut(r, c - 1) = density;
        // *self.density.at_mut(r, c - 2) = density;
        // *self.density.at_mut(r, c - 3) = density;
	}

	// pub fn print_variable(v: &Variable) {
	// 	for i in v.values.iter().rev() {
	// 		for j in i {
	// 			print!("{:.*} ", 2,  *j);
	// 		}
	// 		println!("");
	// 	}
	// 	println!("");
	// }

	pub fn print_velocity(&self) {
		println!("{{");
		for i in 0..self.rows {
			for j in 0..self.columns {
				let v = format!("{}, {}", self.velocity_x.at(i, j), self.velocity_y.at(i, j) );
				let p = format!("{}, {}", j, i );

				print!("{{ {{ {} }}, {{ {} }} }}, ", p, v);
			}
		}
		println!("}}");
	}

	pub fn print_divergence(&self) {
		for i in (0..self.rows).rev() {
			for j in 0..self.columns {
				print!("{:.*}, ", 2, self.divergence.at(i, j) );
			}
			println!("");
		}
		println!("");
	}

	pub fn print_pressure(&self) {
		for i in (0..self.rows).rev() {
			for j in 0..self.columns {
				print!("{:.*}, ", 2, self.pressure.at(i, j));
			}
			println!("");
		}
		println!("");
	}

    pub fn print_density(&self) {
		for i in (0..self.rows).rev() {
			for j in 0..self.columns {
				print!("{:.*}, ", 2, self.density.at(i, j));
			}
			println!("");
		}
		println!("");
	}

	pub fn write_velocity(&self) {
		let mut f = File::create("data.dat").unwrap();

		let mut data = String::new();

		for i in 0..self.rows {
			for j in 0..self.columns {
				let v = format!("{} {}", self.velocity_x.at(i, j), self.velocity_y.at(i, j) );
				let p = format!("{} {}", j, i );

				data = data + &*format!("{} {} ", p, v);
			}
		}

		let _ = f.write_all(data.as_bytes());
	}


	// pub fn variable_image(v: &Variable, name: &str) {
	// 	let mut temp = vec![];
	// 	for i in v.values.iter().rev() {
	// 		for j in i {
	// 			temp.push(*j as u8);
	// 			temp.push(*j as u8);
	// 			temp.push(*j as u8);
	// 			temp.push(*j as u8);
	// 		}
	// 	}
	// 	//let _ = lodepng::encode_file(name, &temp, v.values[0].len() as usize, v.values.len() as usize, lodepng::LCT_RGB, 8);
	// 	let _ = lodepng::encode32_file(name, &temp.as_slice(), v.values[0].len() as usize, v.values.len() as usize);
	// }


}

// fn test() {
// 	let mut solver = FluidSolver::new(1.0, 100, 100, 0.01, 1.0);
// 	solver.solve();
//
// }
