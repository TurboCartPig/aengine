mod gl;
mod shader;

use std::rc::Rc;
use std::ffi::CStr;

use glutin::Window;
use logger::Logger;

type VertexBufferObject = gl::types::GLuint;
type VertexArrayObject = gl::types::GLuint;
type ElementBufferObject = gl::types::GLuint;

#[derive(Debug)]
pub enum RendererCreationError {
    ProgramCreationError(shader::ProgramCreationError),
}

impl From<shader::ProgramCreationError> for RendererCreationError {
    fn from(from: shader::ProgramCreationError) -> Self {
        RendererCreationError::ProgramCreationError(from)
    }
}

#[derive(Debug)]
pub enum GlError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    StackUnderflow,
    StackOverflow,
    ContextLost,
    TableTooLarge,
}

pub fn get_gl_error() -> Result<(), GlError> {
    let error = unsafe {
        gl::GetError()
    };

    match error {
        0x0500u32 => Err(GlError::InvalidEnum),
        0x0501u32 => Err(GlError::InvalidValue),
        0x0502u32 => Err(GlError::InvalidOperation),
        0x0503u32 => Err(GlError::StackOverflow),
        0x0504u32 => Err(GlError::StackUnderflow),
        0x0505u32 => Err(GlError::OutOfMemory),
        0x0506u32 => Err(GlError::InvalidFramebufferOperation),
        0x0507u32 => Err(GlError::ContextLost),
        0x8031u32 => Err(GlError::TableTooLarge),
        _ => Ok(()),
    }
}

pub struct Renderer {
    logger: Rc<Logger>,
    window: Rc<Window>,
    program: shader::Program,
}

impl Renderer {
    pub fn new(window: Rc<Window>, logger: Rc<Logger>) -> Result<Self, RendererCreationError> {
        unsafe {
            window.make_current().unwrap();

            let mut fns = String::new();
            gl::load_with(|sym| {
                let addr = window.get_proc_address(sym) as *const _;

                fns += format!("{:?}\t{:?}\n", addr, sym).as_str();

                addr
            });

            debug!(logger, "OpenGL Version: {:?}", CStr::from_ptr(gl::GetString(gl::VERSION) as *const _));
            debug!(logger, "GLSL Version: {:?}", CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const _));
            //debug!(logger, "OpenGL functions:\n{}", fns);

            gl::ClearColor(1f32, 0f32, 0f32, 1f32);
        }

        let program = unsafe { shader::Program::new()? };

        unsafe {
            gl::UseProgram(*program);
        }

        Ok(Renderer {
            logger,
            window,
            program,
        })
    }

    pub fn render(&mut self, delta_time: f64) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.swap_buffers().unwrap();

        // Print all gl errors
        while let Err(err) = get_gl_error() {
            error!(self.logger, "OpenGL Error: {:?}", err);
        }
    }
}