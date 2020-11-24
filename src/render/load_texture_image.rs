use std::rc::Rc;
use std::cell::RefCell;

use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{
    console,
    HtmlImageElement,
    WebGl2RenderingContext as GL,
};

fn load_placeholder_texture(gl: Rc<GL>) {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let data = [0, 0, 255, 255];
    let data_location = data.as_ptr() as u32;

    let data_array = js_sys::Uint8Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    let target = GL::TEXTURE_2D;
    let level = 0;
    let internal_format = GL::RGBA as i32;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
        target,
        level,
        internal_format,
        width,
        height,
        border,
        src_format,
        src_type,
        &data_array,
        0
    )
    .expect("Texture image 2d");
}

pub fn load_texture_image(gl: Rc<GL>, src: &str) -> Rc<Option<web_sys::WebGlTexture>> {
    load_placeholder_texture(gl.clone());
    let texture = Rc::new(gl.create_texture());

    let image = Rc::new(
        RefCell::new(HtmlImageElement::new().unwrap())
    );
    let image_clone = image.clone();
    let texture_clone = texture.clone();

    let onload = Closure::wrap(Box::new(move || {

        gl.active_texture(GL::TEXTURE0);

        let target = GL::TEXTURE_2D;

        gl.bind_texture(target, (&(*texture_clone)).as_ref());

        gl.generate_mipmap(target);
        // gl.tex_parameteri(target, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        // gl.tex_parameteri(target, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
        // gl.tex_parameteri(target, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);

        let level = 0;
        let internal_format = GL::RGBA as i32;
        let width = 256;
        let height = 256;
        let border = 0;
        let format = GL::RGBA;
        let data_type = GL::UNSIGNED_BYTE;
        let source = &image_clone.borrow();
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            data_type,
            source,
        )
        .expect("Texture image 2d");

        // gl.generate_mipmap(target);

        console::log_2(&"texture loaded".into(), &image_clone.clone().borrow().complete().into());

    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_src(src);

    onload.forget();

    texture
}