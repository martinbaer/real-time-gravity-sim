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
        let p = Bodies {
            particles: Vec::new(),
            canvas_width: 0.0,
            canvas_height: 0.0,
        };
        Mutex::new(p)
    };
}

// Imported JS function from draw_particle.js
#[wasm_bindgen]
extern "C" {
    pub fn draw_particle(x: f64, y: f64, s: &str, size: i32);
}

// Exported Rust functions to be used by initialiser.js
#[wasm_bindgen]
pub fn create_particles(w: f64, h: f64, num: i32) {
    BODIES.lock().unwrap().canvas_width = w;
    BODIES.lock().unwrap().canvas_height = h;
    BODIES.lock().unwrap().create(num);
}
#[wasm_bindgen]
pub fn render_particles() {
    BODIES.lock().unwrap().draw();
    BODIES.lock().unwrap().update();
}
