use crate::application::*;
use std::any::Any;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub struct Shader {
    pub name: String,
    pub vertex_shader: String,
    pub fragment_shader: String,
}

impl Shader {
    fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    pub fn compile_vertex_shader(&self, context: &WebGlRenderingContext) -> Result<WebGlShader, String> {
        return Shader::compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, &self.vertex_shader);
    }

    pub fn compile_fragment_shader(&self, context: &WebGlRenderingContext) -> Result<WebGlShader, String> {
        return Shader::compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, &self.fragment_shader);
    }

    pub fn use_shader(&self, context: &WebGlRenderingContext) -> Result<(), String> {
        let vert = &self.compile_vertex_shader(context)?;
        let frag = &self.compile_fragment_shader(context)?;

        let program = Shader::link_program(&context, &vert, &frag)?;
        context.use_program(Some(&program));

        return Ok(());
    }
}

pub struct Material {
    pub name: String,
    pub shader: String
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<f32>,
}

pub trait System {
    fn execute(&self, app: &mut Application);
}

pub trait Component: Any {
    fn get_name(&self) -> String;
    fn is(&self, name: &str) -> bool;
    fn as_any(&self) -> &dyn Any;
}
