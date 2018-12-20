extern crate lodepng;

use glium;

use glium::{Surface};
use glium::glutin;
use glium::Program;
use glium::VertexBuffer;
use glium::index;
use glium::DrawParameters;
use glium::texture::{RawImage2d, ClientFormat};
use glium::backend::glutin::Display;

use std::borrow::Cow;
use std::f64::consts::PI;
use field::Field;

use interpolation;



#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f64; 2],
    pub colour: [f32; 4]
}

implement_vertex!(Vertex, position, colour);

pub struct Visualiser {
    display: Display,
    program: Program
}

impl Visualiser {
    pub fn new(rows: usize, columns: usize) -> Visualiser {
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            in vec4 colour;

            out vec4 colour_out;

            void main() {
                gl_PointSize = 500.0;
                gl_Position = vec4(position, 0.0, 1.0);
                colour_out = colour;
        }"#;

        let fragment_shader_src = r#"
            #version 140
            out vec4 color;

            in vec4 colour_out;

            void main() {
                color = colour_out;
        }"#;

        let x = 8.0 as u32;
        
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_dimensions( (columns as u32 * x, rows as u32 * x).into() );
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();


        Visualiser {
            program: Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            display: display,
        }
    }

    pub fn to_image(&self, name: &str) {
        let pixels: Vec<Vec<(u8, u8, u8, u8)>> = self.display.read_front_buffer();
        let mut temp = vec![];

        let rows: usize = pixels.len();
        let columns: usize = pixels[0].len();

        for i in 0..rows {
            for j in 0..columns {
                temp.push(pixels[i][j]);
            }
        }

        let _ = lodepng::encode32_file(name, &temp.as_slice(), columns, rows);
    }

    fn grey_to_jet(mut v: f64, min: f64, max: f64) -> (f32, f32, f32) {
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

        let min = pressure.rows as f64 * -4.9;
        let max = pressure.rows as f64 * 4.9;

        let pressure_flat: Vec<f32> = pressure.field.clone().into_iter()
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
                                 ]).unwrap()
    	};
		
        let index_buffer = glium::IndexBuffer::new(&self.display, index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap();
        let program = program!(&self.display, 140 => {
                                    vertex: "
                                    #version 140
                                    uniform mat4 matrix;
                                    in vec2 position;
                                    in vec2 tex_coords;
                                    out vec2 v_tex_coords;
                                    void main() {
                                        gl_Position = matrix * vec4(position, 0.0, 1.0);
                                        v_tex_coords = tex_coords;
                                    }",

			            fragment: "
				    #version 140
				    uniform sampler2D tex;
				    in vec2 v_tex_coords;
				    out vec4 f_color;
				    void main() {
			    	        f_color = texture(tex, v_tex_coords);
				    }"}).unwrap();
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

    pub fn draw_density(&self, density: &Field) {

        let dens: Vec<f32> = density.field
            .clone()
            .into_iter()
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
                    }",

                fragment: "
                    #version 140
                    uniform sampler2D tex;
                    in vec2 v_tex_coords;
                    out vec4 f_color;
                    void main() {
                       f_color = texture(tex, v_tex_coords);
                    }"

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
        let dens: Vec<f32> = density.field
            .clone()
            .into_iter()
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
        let program = program!(&self.display, 140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec2 tex_coords;
                out vec2 v_tex_coords;
                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);\
                    v_tex_coords = tex_coords;
                }",
	    fragment: "
		#version 140
		uniform sampler2D tex;
		in vec2 v_tex_coords;
		out vec4 f_color;
		void main() {
		    f_color = texture(tex, v_tex_coords);
	        }"}).unwrap();

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

        let dens: Vec<f32> = density.field
            .iter()
            .clone()
            .map(|v| { let (r, g, b) = Visualiser::grey_to_jet(*v, min, max); vec![r as f32, g as f32, b as f32] } )
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
        let program = program!(&self.display, 140 => {
            vertex: "
               #version 140
               uniform mat4 matrix;
               in vec2 position;
               in vec2 tex_coords;
               out vec2 v_tex_coords;
               void main() {
                       gl_Position = matrix * vec4(position, 0.0, 1.0);
                       v_tex_coords = tex_coords;
               }",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;
                void main() {
                    f_color = texture(tex, v_tex_coords);
                }"}).unwrap();

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
        }

        let _ = lodepng::encode_file(name, &temp.as_slice(), density.columns as usize, density.rows as usize, lodepng::ColorType::GREY, 8);
    }

    pub fn draw_vector_field(&self, u: &Field, v: &Field, x_resolution: u32, y_resolution: u32) { 
        let cols = v.columns as f64;
        let rows = u.rows as f64;

        let mut ps = Vec::new();

	for x in 0..x_resolution {
            for y in 0..y_resolution {
                let mut u_x = interpolation::bilinear_interpolate(cols * x as f64 / x_resolution as f64, rows * y as f64 / y_resolution as f64, u);
                let mut v_y = interpolation::bilinear_interpolate(cols * x as f64 / x_resolution as f64, rows * y as f64 / y_resolution as f64, v);

                let length_scaler = 28.0;

                let mag = (u_x*u_x + v_y*v_y).sqrt();
                let scale = mag.max(15.0);
                u_x = u_x / scale;
                v_y = v_y / scale;

                let (r, g, b) = Visualiser::grey_to_jet(mag, 0.0, 20.0);

                let (px0, py0) = ( ((2.0 * x as f64)  / x_resolution as f64 ) - 1.0 , ((2.0 * y as f64) / y_resolution as f64) - 1.0 );
                let (px1, py1) = ( ((2.0 * x as f64)  / x_resolution as f64 ) - 1.0 + u_x/length_scaler, ((2.0 * y as f64) / y_resolution as f64) - 1.0 + v_y/length_scaler );
                let angle = (py0 - py1).atan2(px0 - px1);
                let angle0 = angle + (PI / 2.0);
                let angle1 = angle - (PI / 2.0);
                let base = 0.005 * (0.4 + mag/length_scaler);
                let (vx1, vy1) = (px0 + (base * f64::cos(angle0)), py0 + (base * f64::sin(angle0)));
                let (vx2, vy2) = (px0 + (base * f64::cos(angle1)), py0 + (base * f64::sin(angle1)));

                ps.push( Vertex {
                    position: [ vx1, vy1 ],
                    colour: [ r, g, b, 1.0 ]
                } );
                ps.push( Vertex {
                    position: [ vx2, vy2 ],
                    colour: [ r, g, b, 1.0 ]
                } );
                ps.push( Vertex {
                    position: [ px1, py1 ],
                    colour: [ r, g, b, 1.0 ]
                } );
            }
        }

        let vertex_buffer = VertexBuffer::new(&self.display, &ps).unwrap();
        let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn draw_markers(&self, points: &Vec<(f64, f64)>, u: &Field, v: &Field) {
        let width = v.columns as f64;
        let height = u.rows as f64;

	let mut ps = Vec::new();

	for p in points {
            let (x, y) = *p;
            let u_x = interpolation::bilinear_interpolate(x, y, u);
            let v_y = interpolation::bilinear_interpolate(x, y, v);

            let mag = (u_x*u_x + v_y*v_y).sqrt();

            let (r, g, b) = Visualiser::grey_to_jet(mag, 0.0, 20.0);

	    ps.push( Vertex {
                position: [ (2.0 * x  / width as f64 ) - 1.0 , (2.0 * y / height as f64) - 1.0 ],
                colour: [ r, g, b, 1.0 ]
             } )
        }

		let vertex_buffer = VertexBuffer::new(&self.display, &ps).unwrap();
        let indices = index::NoIndices(index::PrimitiveType::Points);


        let params = DrawParameters {
            point_size: Some(5.0),
            .. Default::default()
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target.draw(&vertex_buffer, &indices, &self.program, &glium::uniforms::EmptyUniforms, &params).unwrap();
        target.finish().unwrap();
    }

}
