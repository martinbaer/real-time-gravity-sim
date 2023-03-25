use rand::Rng;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate lazy_static;
//Global variable
lazy_static! {
    static ref PARTICLES: Mutex<Particles> = {
        let p = Particles {
            particles: Vec::new(),
            canvas_width: 0.0,
            canvas_height: 0.0,
        };
        Mutex::new(p)
    };
}
//Particle
pub struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: String,
}

impl Particle {
    fn new(x_starting_range: f64, y_starting_range: f64) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            x: x_starting_range * rng.gen::<f64>(),
            y: y_starting_range * rng.gen::<f64>(),
            vx: 4.0 * rng.gen::<f64>() - 2.0,
            vy: 4.0 * rng.gen::<f64>() - 2.0,
            color: get_random_rgb(),
        }
    }
}
//Particles
pub struct Particles {
    particles: Vec<Particle>,
    canvas_width: f64,
    canvas_height: f64,
}

impl Particles {
    fn create(&mut self, num: i32) {
        for _ in 0..num {
            self.particles
                .push(Particle::new(self.canvas_width, self.canvas_height));
        }
    }

    fn draw(&self) {
        for p in self.particles.iter() {
            draw_particle(p.x, p.y, &p.color, 2);
        }
    }

    fn update(&mut self) {
        for p in self.particles.iter_mut() {
            p.x += p.vx;
            p.y += p.vy;

            if p.x < 0.0 || p.x > self.canvas_width {
                p.vx = -p.vx;
            }

            if p.y < 0.0 || p.y > self.canvas_height {
                p.vy = -p.vy;
            }
        }
    }
}
//Helper function
fn get_random_rgb() -> String {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut rng = rand::thread_rng();

    while r < 100 && g < 100 && b < 100 {
        r = rng.gen_range(0..256);
        g = rng.gen_range(0..256);
        b = rng.gen_range(0..256);
    }
    format!("rgb({r},{g},{b})").to_string()
}
//Imported JS function from index.html
#[wasm_bindgen]
extern "C" {
    fn draw_particle(x: f64, y: f64, s: &str, size: i32);
}
//Exported Rust functions used by index.html
#[wasm_bindgen]
pub fn create_particles(w: f64, h: f64, num: i32) {
    PARTICLES.lock().unwrap().canvas_width = w;
    PARTICLES.lock().unwrap().canvas_height = h;
    PARTICLES.lock().unwrap().create(num);
}

#[wasm_bindgen]
pub fn render_particles() {
    PARTICLES.lock().unwrap().draw();
    PARTICLES.lock().unwrap().update();
}
