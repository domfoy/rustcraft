use std::rc::Rc;

use web_sys:: {
    console,
    WebGl2RenderingContext as GL,
};

use crate::shader::Shader;
use crate::render::render_trait::Render;

pub struct BlockTile<'a> {
    shader: &'a Shader,
}

impl<'a> BlockTile<'a> {
    pub fn new(shader: &'a Shader) -> Self {
        BlockTile{
            shader
        }
    }

    fn shader(&'a self) -> &'a Shader {
        &self.shader
    }

    pub fn buffer_attributes(&self, gl: &GL, texture: Rc<Option<web_sys::WebGlTexture>>) {
        let shader = self.shader();

        let pos_attrib = gl.get_attrib_location(
            &shader.program,
            "a_position"
        );
        let texture_uniform = gl.get_uniform_location(
            &shader.program,
            "u_texture"
        );

        // gl.uniform4f(color_uniform.as_ref(), 0.0, 0.5, 0.1, 1.0);
        let tex_coord_attrib = gl.get_attrib_location(
            &shader.program,
            "a_tex_coord"
        );
        gl.enable_vertex_attrib_array(pos_attrib as u32);
        gl.enable_vertex_attrib_array(tex_coord_attrib as u32);

        gl.active_texture(GL::TEXTURE0);
        gl.bind_texture(GL::TEXTURE_2D, (&(*texture)).as_ref());
        gl.uniform1i(texture_uniform.as_ref(), 0);

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

        let width = 256.;
        // let tex_coords: [f32; 24] = [
        //     0., 0.8, 0.1, 1.,
        //     0.8, 0.8, 0.1, 1.,
        //     0., 0.8, 0.1, 1.,
        //     0., 0.8, 0.1, 1.,
        //     0.8, 0.8, 0.1, 1.,
        //     0., 0.8, 0.1, 1.,
        // ];
        let tex_coords: [f32; 12] = [
            0., 0.,
            1., 0.,
            0., 1.,
            0., 1.,
            1., 0.,
            1., 1.,
        ];
        // let tex_coords: [f32; 12] = [
        //     0. / width, 16. / width,
        //     48. / width, 16. / width,
        //     0. / width, 0. / width,
        //     0. / width, 0. / width,
        //     64. / width, 16. / width,
        //     64. / width, 0. / width,
        // ];

        Self::buffer_f32_data(
            &gl,
            &tex_coords,
            tex_coord_attrib as u32,
            2
        );

        let gl_error_code = gl.get_error();

        console::log_1(&gl_error_code.into());
    }
}

impl<'a> Render for BlockTile<'a> {
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