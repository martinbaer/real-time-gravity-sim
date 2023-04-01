use std::sync::Mutex;
use wasm_bindgen::prelude::*;

mod constants;
mod simulation;

use crate::simulation::Simulation;

#[macro_use]
extern crate lazy_static;
// Global variable for the bodies
lazy_static! {
    static ref BODIES: Mutex<Simulation> = {
        let data = Simulation::new_empty();
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
    BODIES.lock().unwrap().create(num, w, h, is_mobile);
}
#[wasm_bindgen]
pub fn render_bodies() {
    BODIES.lock().unwrap().draw();
    BODIES.lock().unwrap().update();
}
#[wasm_bindgen]
pub fn on_click(x: f64, y: f64) {
    BODIES.lock().unwrap().on_click(x, y);
}
#[wasm_bindgen]
pub fn off_click(x: f64, y: f64) {
    BODIES.lock().unwrap().off_click(x, y);
}
#[wasm_bindgen]
pub fn set_dt(dt: f64) {
    BODIES.lock().unwrap().dt = dt;
}
#[wasm_bindgen]
pub fn set_gravity(gravity: f64) {
    BODIES.lock().unwrap().gravity = gravity;
}
#[wasm_bindgen]
pub fn set_spawn_radius(spawn_radius: f64) {
    BODIES.lock().unwrap().spawner.spawn_radius = spawn_radius;
}
#[wasm_bindgen]
pub fn set_spawn_speed(g: f64) {
    BODIES.lock().unwrap().spawner.spawn_speed = g;
}
