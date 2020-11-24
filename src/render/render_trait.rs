use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;

pub trait Render {
    fn render(&self, gl: &GL);

    fn buffer_f32_data(gl: &GL, data: &[f32], attrib: u32, size: i32) {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let data_location = data.as_ptr() as u32 / 4;

        let data_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        let buffer = gl.create_buffer().unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &data_array,
            GL::STATIC_DRAW
        );

        let data_type = GL::FLOAT;   // the data is 32bit floats
        let normalize = false;       // don't normalize the data
        let stride = 0;              // 0 = move forward size * sizeof(type) each iteration to get the next position
        let offset = 0;              // start at the beginning of the buffer
        gl.vertex_attrib_pointer_with_i32(
            attrib,
            size,
            data_type,
            normalize,
            stride,
            offset
        );
    }
}