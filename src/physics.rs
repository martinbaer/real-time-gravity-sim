use rand::{rngs::ThreadRng, Rng};

mod bh_tree;

use crate::constants::{BODY_DRAW_SIZE, STAR_COLOURS, STAR_COLOURS_LEN};
use crate::draw_body;

// The bodies struct is a Struct of Arrays (SoA) implementation of the bodies
pub struct Bodies {
    pub num_bodies: usize,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub vx: Vec<f64>,
    pub vy: Vec<f64>,
    pub ax: Vec<f64>,
    pub ay: Vec<f64>,
    pub canvas_width: f64,
    pub canvas_height: f64,
    // pub bh_tree: bh_tree::Tree,
}
impl Bodies {
    pub fn new_empty() -> Bodies {
        Bodies {
            num_bodies: 0,
            x: Vec::new(),
            y: Vec::new(),
            vx: Vec::new(),
            vy: Vec::new(),
            ax: Vec::new(),
            ay: Vec::new(),
            canvas_width: 0.0,
            canvas_height: 0.0,
            // bh_tree: bh_tree::Tree::new_empty(),
        }
    }

    pub fn create(&mut self, num: usize) {
        self.num_bodies = num;
        for _ in 0..num {
            let mut rng: ThreadRng = rand::thread_rng();
            self.x.push(self.canvas_width * rng.gen::<f64>());
            self.y.push(self.canvas_height * rng.gen::<f64>());
            self.vx.push(4.0 * rng.gen::<f64>() - 2.0);
            self.vy.push(4.0 * rng.gen::<f64>() - 2.0);
            self.ax.push(0.0);
            self.ay.push(0.0);
        }
    }

    pub fn draw(&self) {
        // draw the bodies with color rotating through STAR_COLOURS
        for i in 0..self.num_bodies {
            let color: &str = STAR_COLOURS[i % STAR_COLOURS_LEN];
            draw_body(self.x[i], self.y[i], color, BODY_DRAW_SIZE);
        }
    }

    pub fn update(&mut self) {
        // update the positions of the bodies
        for i in 0..self.num_bodies {
            self.x[i] += self.vx[i];
            self.y[i] += self.vy[i];

            if self.x[i] < 0.0 || self.x[i] > self.canvas_width {
                self.vx[i] = -self.vx[i];
            }

            if self.y[i] < 0.0 || self.y[i] > self.canvas_height {
                self.vy[i] = -self.vy[i];
            }
        }
    }
}
