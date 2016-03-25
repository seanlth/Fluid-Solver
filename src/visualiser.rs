extern crate lodepng;

use glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, ElementState};
use glium::glutin;
use glium::backend;
use glium::backend::glutin_backend::GlutinFacade;
use glium::Program;
use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index;
use glium::DrawParameters;
use glium::texture::{RawImage2d, ClientFormat};

use fluid_solver::FluidSolver;
use std::ops::Deref;
use std::borrow::Cow;
use std::borrow;
use field::Field;




#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: [f64; 2],
}

implement_vertex!(Vertex, position);

pub struct Visualiser {
	display: GlutinFacade,
	program: Program
}

impl Visualiser {
	pub fn new() -> Visualiser {
		let vertex_shader_src = r#"
			#version 140
		  	in vec2 position;

		  	void main() {
				gl_PointSize = 500.0;
				gl_Position = vec4(position, 0.0, 1.0);
		  	}
		"#;

		let fragment_shader_src = r#"
			#version 140
			out vec4 color;

			uniform vec4 colour;

			void main() {
		 		color = colour;
			}
		"#;

		let d = glutin::WindowBuilder::new().build_glium().unwrap();

		Visualiser {
			program: Program::from_source(&d, vertex_shader_src, fragment_shader_src, None).unwrap(),
			display: d,
		}
	}

    fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32)
    {
        let mut c_r = 1.0;
        let mut c_g = 1.0;
        let mut c_b = 1.0;

        if v < min { v = min; }
        if v > max { v = max; }
        let dv = max - min;

        if v < (min + 0.25 * dv) {
          c_r = 0.0;
          c_g = 4.0 * (v - min) / dv;
        }
        else if v < (min + 0.5 * dv) {
          c_r = 0.0;
          c_b = 1.0 + 4.0 * (min + 0.25 * dv - v) / dv;
        }
        else if v < (min + 0.75 * dv) {
          c_r = 4.0 * (v - min - 0.5 * dv) / dv;
          c_b = 0.0;
        }
        else {
          c_g = 1.0 + 4.0 * (min + 0.75 * dv - v) / dv;
          c_b = 0.0;
        }

        (c_r as f32, c_g as f32, c_b as f32)
    }

    pub fn draw_pressure(&self, pressure: &Field) {

        //let max = pressure.field.iter().cloned().fold(0./0., f64::max);
        //let min = pressure.field.iter().cloned().fold(0./0., f64::min);

        let min = pressure.rows as f64 * -4.9;
        let max = pressure.rows as f64 * 4.9;

		let pressure_flat: Vec<f32> = pressure.clone()
									.map(|v| { let (r, g, b) = Visualiser::grey_to_jet(v, min, max); vec![r as f32, g as f32, b as f32] } )
									.collect::<Vec<Vec<f32>>>()
									.iter()
									.flat_map(|a| a.clone())
									.collect();

		let (w, h) = (pressure.columns as u32, pressure.rows as u32);

		let raw = RawImage2d {
			data: Cow::Owned(pressure_flat),
			width: w,
			height: h,
			format: ClientFormat::F32F32F32
		};

		let opengl_texture = glium::texture::Texture2d::new(&self.display, raw).unwrap();

		let vertex_buffer = {
	        #[derive(Copy, Clone)]
	        struct Vertex2 {
	            position: [f32; 2],
	            tex_coords: [f32; 2],
	        }

	        implement_vertex!(Vertex2, position, tex_coords);

	        glium::VertexBuffer::new(&self.display,
	            &[
	                Vertex2 { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
	                Vertex2 { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
	                Vertex2 { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
	                Vertex2 { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
	            ]
	        ).unwrap()
    	};
		let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
		let program = program!(&self.display,
	    	140 => {
			   vertex: "
				   #version 140
				   uniform mat4 matrix;
				   in vec2 position;
				   in vec2 tex_coords;
				   out vec2 v_tex_coords;
				   void main() {
					   gl_Position = matrix * vec4(position, 0.0, 1.0);
					   v_tex_coords = tex_coords;
				   }
			   ",

			   fragment: "
				   #version 140
				   uniform sampler2D tex;
				   in vec2 v_tex_coords;
				   out vec4 f_color;
				   void main() {
					   f_color = texture(tex, v_tex_coords);
				   }
			   "
		   }).unwrap();
		   let uniforms = uniform! {
		   matrix: [
			   [1.0, 0.0, 0.0, 0.0],
			   [0.0, 1.0, 0.0, 0.0],
			   [0.0, 0.0, 1.0, 0.0],
			   [0.0, 0.0, 0.0, 1.0f32]
		   ],
		   tex: &opengl_texture
	   };

	   // drawing a frame
	   let mut target = self.display.draw();
	   target.clear_color(0.0, 0.0, 0.0, 1.0);
	   target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
	   target.finish().unwrap();


	}

    // pub fn draw_velocity(&self, velocity_x: &Field, velocty_y: &Field) {
    //
	// 	let pressure_flat: Vec<f32> = pressure.clone()
	// 								.map(|v| { let (r, g, b) = Visualiser::grey_to_jet(v, min, max); vec![r as f32, g as f32, b as f32] } )
	// 								.collect::<Vec<Vec<f32>>>()
	// 								.iter()
	// 								.flat_map(|a| a.clone())
	// 								.collect();
    //
	// 	let (w, h) = (pressure.columns as u32, pressure.rows as u32);
    //
	// 	let raw = RawImage2d {
	// 		data: Cow::Owned(pressure_flat),
	// 		width: w,
	// 		height: h,
	// 		format: ClientFormat::F32F32F32
	// 	};
    //
	// 	let opengl_texture = glium::texture::Texture2d::new(&self.display, raw).unwrap();
    //
	// 	let vertex_buffer = {
	//         #[derive(Copy, Clone)]
	//         struct Vertex2 {
	//             position: [f32; 2],
	//             tex_coords: [f32; 2],
	//         }
    //
	//         implement_vertex!(Vertex2, position, tex_coords);
    //
	//         glium::VertexBuffer::new(&self.display,
	//             &[
	//                 Vertex2 { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
	//                 Vertex2 { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
	//                 Vertex2 { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
	//                 Vertex2 { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
	//             ]
	//         ).unwrap()
    // 	};
	// 	let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
	// 	let program = program!(&self.display,
	//     	140 => {
	// 		   vertex: "
	// 			   #version 140
	// 			   uniform mat4 matrix;
	// 			   in vec2 position;
	// 			   in vec2 tex_coords;
	// 			   out vec2 v_tex_coords;
	// 			   void main() {
	// 				   gl_Position = matrix * vec4(position, 0.0, 1.0);
	// 				   v_tex_coords = tex_coords;
	// 			   }
	// 		   ",
    //
	// 		   fragment: "
	// 			   #version 140
	// 			   uniform sampler2D tex;
	// 			   in vec2 v_tex_coords;
	// 			   out vec4 f_color;
	// 			   void main() {
	// 				   f_color = texture(tex, v_tex_coords);
	// 			   }
	// 		   "
	// 	   }).unwrap();
	// 	   let uniforms = uniform! {
	// 	   matrix: [
	// 		   [1.0, 0.0, 0.0, 0.0],
	// 		   [0.0, 1.0, 0.0, 0.0],
	// 		   [0.0, 0.0, 1.0, 0.0],
	// 		   [0.0, 0.0, 0.0, 1.0f32]
	// 	   ],
	// 	   tex: &opengl_texture
	//    };
    //
	//    // drawing a frame
	//    let mut target = self.display.draw();
	//    target.clear_color(0.0, 0.0, 0.0, 1.0);
	//    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
	//    target.finish().unwrap();
    //
    //
	// }

    pub fn draw_density(&self, density: &Field) {

		let dens: Vec<f32> = density.clone()
									.map(|v| { vec![v as f32, v as f32, v as f32] } )
									.collect::<Vec<Vec<f32>>>()
									.iter()
									.flat_map(|a| a.clone())
									.collect();

		let (w, h) = (density.columns as u32, density.rows as u32);

		let raw = RawImage2d {
			data: Cow::Owned(dens),
			width: w,
			height: h,
			format: ClientFormat::F32F32F32
		};

		let opengl_texture = glium::texture::Texture2d::new(&self.display, raw).unwrap();

		let vertex_buffer = {
	        #[derive(Copy, Clone)]
	        struct Vertex2 {
	            position: [f32; 2],
	            tex_coords: [f32; 2],
	        }

	        implement_vertex!(Vertex2, position, tex_coords);

	        glium::VertexBuffer::new(&self.display,
	            &[
	                Vertex2 { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
	                Vertex2 { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
	                Vertex2 { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
	                Vertex2 { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
	            ]
	        ).unwrap()
    	};
		let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
        let program = program!(&self.display,
	    	140 => {
			   vertex: "
				   #version 140
				   uniform mat4 matrix;
				   in vec2 position;
				   in vec2 tex_coords;
				   out vec2 v_tex_coords;
				   void main() {
					   gl_Position = matrix * vec4(position, 0.0, 1.0);
					   v_tex_coords = tex_coords;
				   }
			   ",

			   fragment: "
				   #version 140
				   uniform sampler2D tex;
				   in vec2 v_tex_coords;
				   out vec4 f_color;
				   void main() {
					   f_color = texture(tex, v_tex_coords);
				   }
			   "
        }).unwrap();
        let uniforms = uniform! {
		    matrix: [
			   [1.0, 0.0, 0.0, 0.0],
			   [0.0, 1.0, 0.0, 0.0],
			   [0.0, 0.0, 1.0, 0.0],
			   [0.0, 0.0, 0.0, 1.0f32]
		   ],
		   tex: &opengl_texture
        };

	   // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
	}

    pub fn draw_density_inverse(&self, density: &Field) {

		let dens: Vec<f32> = density.clone()
									.map(|v| { vec![1.0-v as f32, 1.0-v as f32, 1.0-v as f32] } )
									.collect::<Vec<Vec<f32>>>()
									.iter()
									.flat_map(|a| a.clone())
									.collect();

		let (w, h) = (density.columns as u32, density.rows as u32);

		let raw = RawImage2d {
			data: Cow::Owned(dens),
			width: w,
			height: h,
			format: ClientFormat::F32F32F32
		};

		let opengl_texture = glium::texture::Texture2d::new(&self.display, raw).unwrap();

		let vertex_buffer = {
	        #[derive(Copy, Clone)]
	        struct Vertex2 {
	            position: [f32; 2],
	            tex_coords: [f32; 2],
	        }

	        implement_vertex!(Vertex2, position, tex_coords);

	        glium::VertexBuffer::new(&self.display,
	            &[
	                Vertex2 { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
	                Vertex2 { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
	                Vertex2 { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
	                Vertex2 { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
	            ]
	        ).unwrap()
    	};
		let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
        let program = program!(&self.display,
	    	140 => {
			   vertex: "
				   #version 140
				   uniform mat4 matrix;
				   in vec2 position;
				   in vec2 tex_coords;
				   out vec2 v_tex_coords;
				   void main() {
					   gl_Position = matrix * vec4(position, 0.0, 1.0);
					   v_tex_coords = tex_coords;
				   }
			   ",

			   fragment: "
				   #version 140
				   uniform sampler2D tex;
				   in vec2 v_tex_coords;
				   out vec4 f_color;
				   void main() {
					   f_color = texture(tex, v_tex_coords);
				   }
			   "
        }).unwrap();
        let uniforms = uniform! {
		    matrix: [
			   [1.0, 0.0, 0.0, 0.0],
			   [0.0, 1.0, 0.0, 0.0],
			   [0.0, 0.0, 1.0, 0.0],
			   [0.0, 0.0, 0.0, 1.0f32]
		   ],
		   tex: &opengl_texture
        };

	   // drawing a frame
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
	}

    pub fn draw_density_rgb(&self, density: &Field) {

        let min = 0.0;
        let max = density.field.iter().cloned().fold(0./0., f64::max);

		let dens: Vec<f32> = density.clone()
									.map(|v| { let (r, g, b) = Visualiser::grey_to_jet(v, min, max); vec![r as f32, g as f32, b as f32] } )
									.collect::<Vec<Vec<f32>>>()
									.iter()
									.flat_map(|a| a.clone())
									.collect();

		let (w, h) = (density.columns as u32, density.rows as u32);

		let raw = RawImage2d {
			data: Cow::Owned(dens),
			width: w,
			height: h,
			format: ClientFormat::F32F32F32
		};

		let opengl_texture = glium::texture::Texture2d::new(&self.display, raw).unwrap();

		let vertex_buffer = {
	        #[derive(Copy, Clone)]
	        struct Vertex2 {
	            position: [f32; 2],
	            tex_coords: [f32; 2],
	        }

	        implement_vertex!(Vertex2, position, tex_coords);

	        glium::VertexBuffer::new(&self.display,
	            &[
	                Vertex2 { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
	                Vertex2 { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
	                Vertex2 { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
	                Vertex2 { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
	            ]
	        ).unwrap()
    	};
		let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
		let program = program!(&self.display,
	    	140 => {
			   vertex: "
				   #version 140
				   uniform mat4 matrix;
				   in vec2 position;
				   in vec2 tex_coords;
				   out vec2 v_tex_coords;
				   void main() {
					   gl_Position = matrix * vec4(position, 0.0, 1.0);
					   v_tex_coords = tex_coords;
				   }
			   ",

			   fragment: "
				   #version 140
				   uniform sampler2D tex;
				   in vec2 v_tex_coords;
				   out vec4 f_color;
				   void main() {
					   f_color = texture(tex, v_tex_coords);
				   }
			   "
		   }).unwrap();
		   let uniforms = uniform! {
		   matrix: [
			   [1.0, 0.0, 0.0, 0.0],
			   [0.0, 1.0, 0.0, 0.0],
			   [0.0, 0.0, 1.0, 0.0],
			   [0.0, 0.0, 0.0, 1.0f32]
		   ],
		   tex: &opengl_texture
	   };

	   // drawing a frame
	   let mut target = self.display.draw();
	   target.clear_color(0.0, 0.0, 0.0, 1.0);
	   target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
	   target.finish().unwrap();


	}

	pub fn draw_density_image(&self, density: &Field, name: &str) {
		let mut temp = vec![];
		for i in (0..density.field.len()).rev() {
            temp.push(255 - ( density.field[i] ) as u8 );
            //temp.push( (*i ) as u8 );
            //temp.push( (*i ) as u8 );
            //temp.push( (*i * 1000.0) as u8 );
		}

		//let _ = lodepng::encode24_file(name, &temp.as_slice(), density.columns as usize, density.rows as usize);
        let _ = lodepng::encode_file(name, &temp.as_slice(), density.columns as usize, density.rows as usize, lodepng::LCT_GREY, 8);
	}

	pub fn draw_markers(&self, points: &Vec<(f64, f64)>, width: usize, height: usize) {

		let (pw, ph): (u32, u32) = self.display.get_window().unwrap().deref().get_inner_size_pixels().unwrap();

		let mut ps = Vec::new();

		for p in points {
			let (x, y) = *p;
			ps.push( Vertex { position: [ (2.0 * x  / width as f64 ) - 1.0 , (2.0 * y / height as f64) - 1.0 ] } )
		}


		let c: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
		//let c1: [f32; 4] = if c { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] };

		let vertex_buffer = VertexBuffer::new(&self.display, &ps).unwrap();
        let indices = index::NoIndices(index::PrimitiveType::Points);


		let params = DrawParameters {
    		point_size: Some(5.0),
    		.. Default::default()
		};

		let mut target = self.display.draw();
        target.clear_color(0.2, 0.2, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &self.program, &uniform! { colour: c  }, &params).unwrap();
        target.finish().unwrap();
	}

}
