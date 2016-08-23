use opencl;
use opencl::mem::CLBuffer;
use opencl::hl::*;

pub struct OpenCLKernel {
    pub device: Device,
    pub ctx: Context,
    pub queue: CommandQueue,
    pub kernel: Kernel,
    pub buffers: Vec<CLBuffer<f32>>
}
impl OpenCLKernel {
    pub fn new(kernel: &str) -> Option<OpenCLKernel> {
        if let Ok((device, ctx, queue)) = opencl::util::create_compute_context_using_device(2) {
            let ker = include_str!("kernels.cl");

            let program = ctx.create_program_from_source(ker);
            let info = program.build(&device);
            if let Result::Err(s) = info {
                println!("{}", s);
                return None;
            }

            let kernel = program.create_kernel(kernel);
            return Some(OpenCLKernel {
                device: device,
                ctx: ctx,
                queue: queue,
                kernel: kernel,
                buffers: vec![]
            })
        }
        None
    }

    pub fn create_buffer(&mut self, size: usize, mem_type: opencl::cl::cl_bitfield) {
        let t = self.ctx.create_buffer(size, mem_type);
        self.buffers.push(t);
    }
}
