use std::rc::Rc;

use console_error_panic_hook;
use wasm_bindgen::{
    JsCast,
    prelude::*,
};
use web_sys::{
    HtmlCanvasElement,
    HtmlElement,
    WebGl2RenderingContext as GL,
    window,
};

mod render;
use render::{
    load_texture_image,
    WebRenderer,
};

pub(crate) mod shader;

pub static APP_DIV_ID: &'static str = "scene";

pub static CANVAS_WIDTH: i32 = 512;
pub static CANVAS_HEIGHT: i32 = 512;

fn init_canvas() -> Result<HtmlCanvasElement, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()?;

    canvas.set_width(CANVAS_WIDTH as u32);
    canvas.set_height(CANVAS_HEIGHT as u32);

    let app_div: HtmlElement = match document.get_element_by_id(APP_DIV_ID) {
        Some(container) => container.dyn_into()?,
        None => {
            let app_div = document.create_element("div")?;
            app_div.set_id(APP_DIV_ID);
            app_div.dyn_into()?
        }
    };

    app_div.style().set_property("display", "flex")?;
    app_div.append_child(&canvas)?;

    Ok(canvas)
}

pub fn create_webgl_context() -> Result<GL, JsValue> {
    let canvas = init_canvas()?;

    let gl: GL = canvas.get_context("webgl2")?.unwrap().dyn_into()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.enable(GL::DEPTH_TEST);

    Ok(gl)
}

#[wasm_bindgen]
pub struct WebClient {
    renderer: WebRenderer,
    gl: Rc<GL>,
}

#[wasm_bindgen]
impl WebClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebClient {
        console_error_panic_hook::set_once();

        let gl = Rc::new(create_webgl_context().unwrap());
        let renderer = WebRenderer::new(&gl);

        WebClient {
            gl,
            renderer,
        }
    }

    pub fn start(&mut self) {
        let gl = &self.gl;

        self.renderer.texture = load_texture_image(
            Rc::clone(gl),
            "img/blocks.png",
        );
    }
    pub fn render(&mut self) {
        self.renderer
            .render(&self.gl);
    }
}

