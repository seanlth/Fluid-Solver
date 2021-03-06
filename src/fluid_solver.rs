extern crate lodepng;

use std::io::prelude::*;
use std::fs::File;

use field::Field;
use linear_solvers;
use integrators;
use advection;
use interpolation;



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
    advection: fn(&mut Field, &Field, &Field, f64, f64, &Fn(f64, f64, &Field) -> f64, &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64),
    interpolation: fn(x: f64, y: f64, field: &Field) -> f64,
    integration: fn(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64,
    linear_solver: fn(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize),
}

impl FluidSolver {
    pub fn new(fluid_density: f64, rows: usize, columns: usize, dt: f64, dx: f64, gravity: f64) -> FluidSolver  {
        FluidSolver {
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
        }
    }

    pub fn use_markers(mut self) -> Self {
        for i in 0..self.rows*2 {
            for j in 0..self.columns*2 {
                self.particles.push( (j as f64 * self.dx / 2.0, i as f64*self.dx / 2.0 ) );
            }
        }
        self
    }

    pub fn advection(mut self, f: fn(&mut Field, &Field, &Field, f64, f64, &Fn(f64, f64, &Field) -> f64, &Fn(f64, f64, &Fn(f64, f64) -> f64, f64) -> f64) ) -> Self {
        self.advection = f;
        self
    }

    pub fn interpolation(mut self, f: fn(x: f64, y: f64, field: &Field) -> f64) -> Self {
        self.interpolation = f;
        self
    }

    pub fn integration(mut self, f: fn(x: f64, t: f64, f: &Fn(f64, f64) -> f64, dt: f64) -> f64) -> Self {
        self.integration = f;
        self
    }

    pub fn linear_solver(mut self, f: fn(x: &mut Field, b: &Field, density: f64, dt: f64, dx: f64, limit: usize) ) -> Self {
        self.linear_solver = f;
        self
    }

	// LP = D
	// see diagram
    pub fn pressure_solve(&mut self) {
        (self.linear_solver)( &mut self.pressure, &self.divergence, self.fluid_density, self.dt, self.dx, 600 );
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

            let f1 = |o: f64, _: f64| interpolation::bicubic_interpolate(o, y, &u);
            let f2 = |o: f64, _: f64| interpolation::bicubic_interpolate(x, o, &v);

            let x = (self.integration)(x, 0.0, &f1, self.dt);
            let y = (self.integration)(y, 0.0, &f2, self.dt);
            *p = (x, y);

        }
    }

    pub fn advect(&mut self) {

        let u = self.velocity_x.clone();
        let v = self.velocity_y.clone();

        (self.advection)(&mut self.velocity_x, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration);
        (self.advection)(&mut self.velocity_y, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration);
        (self.advection)(&mut self.density, &u, &v, self.dt, self.dx, &self.interpolation, &self.integration);

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


    pub fn add_source(&mut self, r: usize, c: usize, velocity_x: f64, velocity_y: f64, density: f64) {
        *self.velocity_x.at_mut(r, c) = velocity_x;
        *self.velocity_y.at_mut(r, c) = velocity_y;        
        *self.density.at_mut(r, c) = density;
    }

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
}
