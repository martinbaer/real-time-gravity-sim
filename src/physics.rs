use rand::{rngs::ThreadRng, Rng};

mod bh_tree;

use crate::constants::{BODY_DRAW_SIZE, STAR_COLOURS, STAR_COLOURS_LEN};
use crate::draw_particle;

pub struct Body {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    color: String,
}

impl Body {
    fn new_random(x_starting_range: f64, y_starting_range: f64) -> Body {
        let mut rng: ThreadRng = rand::thread_rng();

        Body {
            x: x_starting_range * rng.gen::<f64>(),
            y: y_starting_range * rng.gen::<f64>(),
            vx: 4.0 * rng.gen::<f64>() - 2.0,
            vy: 4.0 * rng.gen::<f64>() - 2.0,
            // random color from STAR_COLOURS
            color: STAR_COLOURS[rng.gen_range(0..STAR_COLOURS_LEN)].to_string(),
            // color: STAR_COLOURS[5].to_string(),
        }
    }
    fn new_given(x: f64, y: f64, vx: f64, vy: f64) -> Body {
        let mut rng: ThreadRng = rand::thread_rng();

        Body {
            x: x,
            y: y,
            vx: vx,
            vy: vy,
            color: STAR_COLOURS[rng.gen_range(0..STAR_COLOURS_LEN)].to_string(),
        }
    }
}
//Particles
pub struct Bodies {
    pub particles: Vec<Body>,
    pub canvas_width: f64,
    pub canvas_height: f64,
}

impl Bodies {
    pub fn create(&mut self, num: i32) {
        for _ in 0..num {
            self.particles
                .push(Body::new_random(self.canvas_width, self.canvas_height));
        }
    }

    pub fn draw(&self) {
        for p in self.particles.iter() {
            draw_particle(p.x, p.y, &p.color, BODY_DRAW_SIZE);
        }
    }

    pub fn update(&mut self) {
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
