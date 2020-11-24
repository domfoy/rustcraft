use std::rc::Rc;

use web_sys:: {
    WebGl2RenderingContext as GL,
};

use crate::shader::{
    ShaderKind,
    ShaderSystem,
};
use crate::{
    CANVAS_HEIGHT,
    CANVAS_WIDTH,
};

mod tile;
mod render_trait;
mod load_texture_image;

pub use load_texture_image::load_texture_image;

use render_trait::Render;

pub struct WebRenderer {
    shader_sys: ShaderSystem,
    pub texture: Rc<Option<web_sys::WebGlTexture>>,
}

fn prepare_for_render<'a>(
    gl: &GL
) {
    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());
}

fn render_basic(web_renderer: &mut WebRenderer, gl: &GL) {
    gl.bind_framebuffer(GL::FRAMEBUFFER, None);

    let basic_shader = web_renderer
        .shader_sys
        .get_shader(&ShaderKind::Basic)
        .unwrap();
    web_renderer
        .shader_sys
        .use_program(gl, ShaderKind::Basic);

    let basic_tile = tile::BasicTile::new(basic_shader);

    prepare_for_render(gl);
    basic_tile.buffer_attributes(gl);
    basic_tile.render(gl);
}

fn render_block(web_renderer: &mut WebRenderer, gl: &GL) {
    gl.bind_framebuffer(GL::FRAMEBUFFER, None);

    let block_shader = web_renderer
        .shader_sys
        .get_shader(&ShaderKind::Block)
        .unwrap();
    web_renderer
        .shader_sys
        .use_program(gl, ShaderKind::Block);

    let block_tile = tile::BlockTile::new(block_shader);

    prepare_for_render(gl);
    block_tile.buffer_attributes(gl, web_renderer.texture.clone());
    block_tile.render(gl);
}

impl WebRenderer {
    pub fn new(gl: &GL) -> Self {
        let shader_sys = ShaderSystem::new(&gl);

        WebRenderer{
            shader_sys,
            texture: Rc::new(None),
        }
    }

    pub fn render(&mut self, gl: &GL) {
        gl.clear_color(0., 0., 0., 0.);
        gl.clear_depth(1.);

        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        gl.enable(GL::DEPTH_TEST);

        gl.enable(GL::CULL_FACE);

        gl.viewport(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);

        // render_basic(self, gl);
        render_block(self, gl);
    }
}