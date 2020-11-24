use web_sys:: {
    WebGl2RenderingContext as GL,
};

use crate::shader::Shader;
use crate::render::render_trait::Render;

pub struct BasicTile<'a> {
    shader: &'a Shader,
}

impl<'a> BasicTile<'a> {
    pub fn new(shader: &'a Shader) -> Self {
        BasicTile{
            shader
        }
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    pub fn buffer_attributes(&self, gl: &GL) {
        let shader = self.shader();

        let pos_attrib = gl.get_attrib_location(
            &shader.program,
            "a_position"
        );
        gl.enable_vertex_attrib_array(pos_attrib as u32);

        let vertices: [f32; 12] = [
            0., 0.,
            0.5, 0.,
            0., 0.5,
            0., 0.5,
            0.5, 0.,
            0.5, 0.5,
        ];

        Self::buffer_f32_data(
            &gl,
            &vertices,
            pos_attrib as u32,
            2
        );
    }
}

impl<'a> Render for BasicTile<'a> {
    fn render(&self, gl: &GL) {
        let primitive_type = GL::TRIANGLES;
        let offset = 0;
        let count = 6;

        gl.draw_arrays(
            primitive_type,
            offset,
            count,
        )
    }
}