
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

	pub fn draw_density(&self, density: &Vec<Vec<f64>>) {

		let dens: Vec<f32> = density.iter()
						     		.flat_map(|a| a.iter().map(|v| vec![*v as f32, *v as f32, *v as f32] ) )
						  			.collect::<Vec<Vec<f32>>>()
									.iter()
									.flat_map(|a| a.clone() )
									.collect();



		let (w, h) = (density[0].len() as u32, density.len() as u32);

		// println!("{:?}", dens.len());
		// println!("{:?}", w);
		// println!("{:?}", h);

		//let tex = RawImage2d::from_raw_rgba_reversed(dens, (w, h) );
		let raw = RawImage2d {
			data: Cow::Owned(dens),
			width: w,
			height: h,
			format: ClientFormat::F32F32F32
		};
		//let tex = RawImage2d::from_raw(data: Cow<[(u8, u8, u8, u8)]>, width: u32, height: u32);
		//let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&self.display, tex).unwrap();
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
	   target.clear_color(0.2, 0.2, 1.0, 1.0);
	   target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
	   target.finish().unwrap();


	}

	pub fn draw_markers(&self, points: &Vec<(f64, f64)>, width: i32, height: i32) {

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
