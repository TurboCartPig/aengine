use std::ffi::CStr;
use std::ptr::{null, null_mut};
use std::ops::Deref;

use renderer::gl;
use renderer::gl::types::{GLenum};
use renderer::get_gl_error;

#[derive(Debug)]
pub enum ProgramCreationError {
    ShaderCompilationError(String),
    ProgramLinkError(String),
    //OtherGlError(GlError),
}

pub struct Shader {
    shader: u32,
}

impl Shader {
    pub unsafe fn new(src: String, shader_type: GLenum) -> Result<Self, ProgramCreationError> {
        let src = [src.as_ptr() as *const _].as_ptr();

        let new = Self {
            shader: gl::CreateShader(shader_type),
        };

        gl::ShaderSource(*new, 1, src, null());
        gl::CompileShader(*new);

        let mut success = 0;
        gl::GetShaderiv(*new, gl::COMPILE_STATUS, &mut success);
        
        if success == 0 {
            let mut max_len = 0i32;
            gl::GetShaderiv(*new, gl::INFO_LOG_LENGTH, &mut max_len);
            
            let mut log = Vec::with_capacity(max_len as usize);
            gl::GetShaderInfoLog(*new, max_len, null_mut(), log.as_mut_ptr());

            let log = CStr::from_ptr(log.as_slice().as_ptr()).to_string_lossy().into_owned();
            return Err(ProgramCreationError::ShaderCompilationError(log));
        }

        Ok(new)
    }
}

impl Deref for Shader {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.shader
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.shader);
        }
    }
}

pub struct Program {
    program: u32,
}

impl Program {
    pub unsafe fn new() -> Result<Self, ProgramCreationError> {
        let vert_src = ::load_file("shaders/def.vert").unwrap();
        let frag_src = ::load_file("shaders/def.frag").unwrap();

        let vert = Shader::new(vert_src, gl::VERTEX_SHADER)?;
        let frag = Shader::new(frag_src, gl::FRAGMENT_SHADER)?;
        
        let new = Self {
            program:  gl::CreateProgram(),
        };
        
        gl::AttachShader(*new, *vert);
        gl::AttachShader(*new, *frag);
        gl::LinkProgram(*new);

        let mut success = 0;
        gl::GetProgramiv(*new, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut max_len = 0i32;
            gl::GetProgramiv(*new, gl::INFO_LOG_LENGTH, &mut max_len);

            let mut log = Vec::with_capacity(max_len as usize);
            gl::GetProgramInfoLog(*new, max_len, null_mut(), log.as_mut_ptr());

            let log = CStr::from_ptr(log.as_slice().as_ptr()).to_string_lossy().into_owned();
            return Err(ProgramCreationError::ProgramLinkError(log));
        }

        Ok(new)
    }
}

impl Deref for Program {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}