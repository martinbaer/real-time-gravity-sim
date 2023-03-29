use std::sync::Mutex;
use wasm_bindgen::prelude::*;

mod constants;
mod physics;

use crate::physics::Bodies;

#[macro_use]
extern crate lazy_static;
// Global variable for the bodies
lazy_static! {
    static ref BODIES: Mutex<Bodies> = {
        let data = Bodies::new_empty();
        Mutex::new(data)
    };
}

// Imported JS function from draw_body.js
#[wasm_bindgen]
extern "C" {
    pub fn draw_body(x: f64, y: f64, color: &str, size: i32);
    // for logging
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);
}

// Exported Rust functions to be used by initialiser.js
#[wasm_bindgen]
pub fn create_bodies(w: f64, h: f64, num: usize, is_mobile: bool) {
    BODIES.lock().unwrap().canvas_width = w;
    BODIES.lock().unwrap().canvas_half_width = w / 2.0;
    BODIES.lock().unwrap().canvas_height = h;
    BODIES.lock().unwrap().canvas_half_height = h / 2.0;
    BODIES.lock().unwrap().is_mobile = is_mobile;
    BODIES.lock().unwrap().create(num);
}
#[wasm_bindgen]
pub fn render_bodies() {
    BODIES.lock().unwrap().draw();
    BODIES.lock().unwrap().update();
}
