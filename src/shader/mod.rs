use std::collections::HashMap;
use std::cell::RefCell;

use wasm_bindgen::{
    JsValue
};
use web_sys::{
    WebGl2RenderingContext as GL,
    WebGlProgram,
};

mod lib;
use lib::*;

static BASIC_VS: &'static str = include_str!("glsl/basic-vertex.glsl");
static BASIC_FS: &'static str = include_str!("glsl/basic-fragment.glsl");

static BLOCK_VS: &'static str = include_str!("glsl/block-vertex.glsl");
static BLOCK_FS: &'static str = include_str!("glsl/block-fragment.glsl");

pub struct Shader {
    pub program: WebGlProgram,
}

impl Shader {
    fn new(
        gl: &GL,
        vert_shader: &str,
        frag_shader: &str,
    ) -> Result<Shader, JsValue> {
        let vert_shader = compile_shader(
            &gl,
            GL::VERTEX_SHADER,
            vert_shader
        )?;
        let frag_shader = compile_shader(
            &gl,
            GL::FRAGMENT_SHADER,
            frag_shader
        )?;
        let program = create_program(
            &gl,
            &vert_shader,
            &frag_shader
        )?;

        Ok(Shader { program })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShaderKind {
    Basic,
    Block,
}

pub struct ShaderSystem {
    programs: HashMap<ShaderKind, Shader>,
    active_program: RefCell<ShaderKind>
}

impl ShaderSystem {
    pub fn new(gl: &GL) -> ShaderSystem {
        let mut programs = HashMap::new();

        let basic_shader = Shader::new(&gl, BASIC_VS, BASIC_FS).unwrap();
        let block_shader = Shader::new(&gl, BLOCK_VS, BLOCK_FS).unwrap();

        let active_program = RefCell::new(ShaderKind::Block);
        gl.use_program(Some(&block_shader.program));

        programs.insert(ShaderKind::Basic, basic_shader);
        programs.insert(ShaderKind::Block, block_shader);

        ShaderSystem {
            programs,
            active_program,
        }
    }

    pub fn get_shader(&self, shader_kind: &ShaderKind) -> Option<&Shader> {
        self.programs.get(shader_kind)
    }

    pub fn use_program(&self, gl: &GL, shader_kind: ShaderKind) {
        if *self.active_program.borrow() == shader_kind {
            return;
        }

        gl.use_program(Some(&self.get_shader(&shader_kind).unwrap().program));
        *self.active_program.borrow_mut() = shader_kind;
    }
}