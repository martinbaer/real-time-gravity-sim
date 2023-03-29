use rand::{rngs::ThreadRng, Rng};

mod bh_tree;
mod calc_acc;
mod logger;

use crate::constants::{
    BODY_DRAW_SIZE, BODY_DRAW_SIZE_MOBILE, GRAVITY, ROOT_NODE_INDEX, STAR_COLOURS, STAR_COLOURS_LEN,
};
use crate::{draw_body, log, log_u32};

use self::bh_tree::Tree;
use self::logger::log_energy;

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
    pub canvas_half_width: f64,
    pub canvas_half_height: f64,
    pub bh_tree: bh_tree::Tree,
    pub com_distances: Vec<f64>,
    pub is_mobile: bool,
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
            canvas_half_width: 0.0,
            canvas_half_height: 0.0,
            bh_tree: Tree::new(),
            com_distances: Vec::new(),
            is_mobile: false,
        }
    }

    pub fn create(&mut self, num: usize) {
        self.num_bodies = num;
        self.x.reserve(num);
        self.y.reserve(num);
        self.vx.reserve(num);
        self.vy.reserve(num);
        self.ax.reserve(num);
        self.ay.reserve(num);
        self.com_distances.reserve(num);
        for _ in 0..num {
            let mut rng: ThreadRng = rand::thread_rng();
            self.x.push(self.canvas_width * rng.gen::<f64>());
            self.y.push(self.canvas_height * rng.gen::<f64>());
            // self.vx.push(4.0 * rng.gen::<f64>() - 2.0);
            // self.vy.push(4.0 * rng.gen::<f64>() - 2.0);
            self.vx.push(0.0);
            self.vy.push(0.0);
            self.ax.push(0.0);
            self.ay.push(0.0);
            self.com_distances.push(0.0);
        }
        // Set the half-width of the root node to the largest dimension of the system
        self.bh_tree.root_half_width = self.canvas_width.max(self.canvas_height) * 5.0; // 2.0;
                                                                                        // Set the centre of the root node to the centre of the system
        self.bh_tree.root_centre = (self.canvas_width / 2.0, self.canvas_height / 2.0);
    }

    fn get_com(&self) -> (f64, f64) {
        let mut com_x: f64 = 0.0;
        let mut com_y: f64 = 0.0;
        for i in 0..self.num_bodies {
            com_x += self.x[i];
            com_y += self.y[i];
        }
        com_x /= self.num_bodies as f64;
        com_y /= self.num_bodies as f64;
        (com_x, com_y)
    }

    // get the 99th percentile of the distance from the centre of mass
    // also uses this to update the root_centre and root_half_width
    fn get_99th_percentile(&mut self, com: (f64, f64)) -> f64 {
        // Find the 99th percentile of the distance from the centre of mass
        for i in 0..self.num_bodies {
            let dx: f64 = self.x[i] - com.0;
            let dy: f64 = self.y[i] - com.1;
            self.com_distances[i] = (dx * dx + dy * dy).sqrt();
        }
        self.com_distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // update self.root_centre and self.root_half_width
        self.bh_tree.root_centre = (com.0, com.1);
        self.bh_tree.root_half_width = self.com_distances[self.num_bodies - 1] * 2.0;
        // return the 99th percentile of the distance from the centre of mass
        self.com_distances[(0.99 * self.num_bodies as f64) as usize]
    }

    pub fn draw(&mut self) {
        let com: (f64, f64) = self.get_com();
        let percentile: f64 = self.get_99th_percentile(com);
        for i in 0..self.num_bodies {
            // calculate the canvas position of the body such that the centre of mass is at the centre of the canvas (canvas_half_width, canvas_half_height) and 99% of the bodies are inside the canvas
            let canvas_x: f64 = (self.x[i] - com.0) * self.canvas_width / (2.0 * percentile)
                + self.canvas_width / 2.0;
            let canvas_y: f64 = (self.y[i] - com.1) * self.canvas_height / (2.0 * percentile)
                + self.canvas_height / 2.0;
            let color: &str = STAR_COLOURS[i % STAR_COLOURS_LEN];
            // if inside the canvas, draw the body
            if canvas_x >= 0.0
                && canvas_x <= self.canvas_width
                && canvas_y >= 0.0
                && canvas_y <= self.canvas_height
            {
                draw_body(
                    canvas_x,
                    canvas_y,
                    color,
                    if self.is_mobile {
                        BODY_DRAW_SIZE_MOBILE
                    } else {
                        BODY_DRAW_SIZE
                    },
                );
            }
        }
        // print percentile * 2
        // log(&format!("Width (AU): {}", percentile * 2.0));
    }

    pub fn update(&mut self) {
        // Re-construct the Barnes-Hut tree
        self.bh_tree.construct(&self.x, &self.y, self.num_bodies);
        // Compute the acceleration for each body
        for i in 0..self.num_bodies {
            let (new_ax, new_ay): (f64, f64) = calc_acc::add_node_acceleration(
                (&self.x[i], &self.y[i]),
                &self.bh_tree,
                ROOT_NODE_INDEX,
                self.bh_tree.root_half_width,
            );
            self.ax[i] = new_ax;
            self.ay[i] = new_ay;
        }

        // Update the velocity and position for each body
        for i in 0..self.num_bodies {
            self.vx[i] += self.ax[i] * GRAVITY;
            self.vy[i] += self.ay[i] * GRAVITY;
            self.x[i] += self.vx[i];
            self.y[i] += self.vy[i];
        }

        // log enegy
        log_energy(&self.x, &self.y, &self.vx, &self.vy, self.num_bodies);
    }
}
