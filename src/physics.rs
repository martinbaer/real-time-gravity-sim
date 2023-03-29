use rand::{rngs::ThreadRng, Rng};

mod bh_tree;
mod calc_acceleration;
mod energy_conservation;

use crate::constants::{
    BODY_DRAW_SIZE, BODY_DRAW_SIZE_MOBILE, DT, GRAVITY, ROOT_NODE_INDEX, SPAWNED_BODY_SPEED,
    SPAWNED_BODY_SPEED_MOBILE, SPAWN_BODY_COLOR, SPAWN_BODY_DRAW_SIZE, SPAWN_BODY_DRAW_SIZE_MOBILE,
    START_BOX_SIZE, STAR_COLOURS, STAR_COLOURS_LEN,
};
use crate::{draw_body, log, log_u32};

use self::bh_tree::Tree;
use self::energy_conservation::log_energy;

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
    clicked: bool,
    mouse_x: f64,
    mouse_y: f64,
    spawned_x: Vec<f64>,
    spawned_y: Vec<f64>,
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
            clicked: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
            spawned_x: Vec::new(),
            spawned_y: Vec::new(),
        }
    }
    pub fn on_click(&mut self, x: f64, y: f64) {
        self.clicked = true;
        self.mouse_x = x;
        self.mouse_y = y;
    }
    pub fn off_click(&mut self, x: f64, y: f64) {
        self.clicked = false;
        // calculate the velocity of the spawned bodies
        let dx = x - self.mouse_x;
        let dy = y - self.mouse_y;
        // scale the disance
        let com: (f64, f64) = self.get_com();
        let percentile: f64 = self.get_99th_percentile(com);
        let scale: f64 = self.canvas_width / (2.0 * percentile);
        let dx = dx / scale;
        let dy = dy / scale;
        let spawn_body_speed = if self.is_mobile {
            SPAWNED_BODY_SPEED_MOBILE
        } else {
            SPAWNED_BODY_SPEED
        };
        let vx = dx * spawn_body_speed;
        let vy = dy * spawn_body_speed;
        // add all spawned bodies to the system
        for i in 0..self.spawned_x.len() {
            self.x.push(self.spawned_x[i]);
            self.y.push(self.spawned_y[i]);
            self.vx.push(vx);
            self.vy.push(vy);
            self.ax.push(0.0);
            self.ay.push(0.0);
            self.com_distances.push(0.0);
        }
        self.spawned_x.clear();
        self.spawned_y.clear();
        self.num_bodies = self.x.len();
        self.bh_tree = Tree::new();
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
            // self.x.push(START_BOX_SIZE * rng.gen::<f64>());
            // self.y.push(START_BOX_SIZE * rng.gen::<f64>());
            // same as above but uniform distribution
            self.x.push(rng.gen_range(0.0..START_BOX_SIZE));
            self.y.push(rng.gen_range(0.0..START_BOX_SIZE));

            // self.vx.push(4.0 * rng.gen::<f64>() - 2.0);
            // self.vy.push(4.0 * rng.gen::<f64>() - 2.0);
            self.vx.push(0.0);
            self.vy.push(0.0);
            self.ax.push(0.0);
            self.ay.push(0.0);
            self.com_distances.push(0.0);
        }
        // Set the half-width of the root node to the largest dimension of the system
        self.bh_tree.root_half_width = self.canvas_width.max(self.canvas_height) / 2.0;
        // Set the centre of the root node to the centre of the system
        self.bh_tree.root_centre = (self.canvas_width / 2.0, self.canvas_height / 2.0);
    }

    pub fn draw(&mut self) {
        let com: (f64, f64) = self.get_com();
        let percentile: f64 = self.get_99th_percentile(com);
        let scale: f64 = self.canvas_width / (2.0 * percentile);
        for i in 0..self.num_bodies {
            // calculate the canvas position of the body such that the centre of mass is at the centre of the canvas (canvas_half_width, canvas_half_height) and 99% of the bodies are inside the canvas
            let canvas_x: f64 = (self.x[i] - com.0) * scale + self.canvas_half_width;
            let canvas_y: f64 = (self.y[i] - com.1) * scale + self.canvas_half_height;
            let color: &str = STAR_COLOURS[i % STAR_COLOURS_LEN];
            let body_draw_size = if self.is_mobile {
                BODY_DRAW_SIZE_MOBILE
            } else {
                BODY_DRAW_SIZE
            };
            // if inside the canvas, draw the body
            if canvas_x >= 0.0
                && canvas_x <= self.canvas_width
                && canvas_y >= 0.0
                && canvas_y <= self.canvas_height
            {
                draw_body(canvas_x, canvas_y, color, body_draw_size);
            }
        }
        // draw spawned bodies (if any)
        for i in 0..self.spawned_x.len() {
            let canvas_x: f64 = (self.spawned_x[i] - com.0) * scale + self.canvas_half_width;
            let canvas_y: f64 = (self.spawned_y[i] - com.1) * scale + self.canvas_half_height;
            let color: &str = SPAWN_BODY_COLOR;
            let body_draw_size = if self.is_mobile {
                SPAWN_BODY_DRAW_SIZE_MOBILE
            } else {
                SPAWN_BODY_DRAW_SIZE
            };
            if canvas_x >= 0.0
                && canvas_x <= self.canvas_width
                && canvas_y >= 0.0
                && canvas_y <= self.canvas_height
            {
                draw_body(canvas_x, canvas_y, color, body_draw_size);
            }
        }

        // print percentile * 2
        // log(&format!("Width (AU): {}", percentile * 2.0));
    }

    // spawn bodies randomly around the click position
    fn spawn_body(&mut self, click_x: f64, click_y: f64) {
        // get scale to convert from canvas to AU
        let com: (f64, f64) = self.get_com();
        let percentile: f64 = self.get_99th_percentile(com);
        let scale: f64 = self.canvas_width / (2.0 * percentile);
        // convert click position from canvas to AU
        let click_x_au: f64 = (click_x - self.canvas_half_width) / scale + com.0;
        let click_y_au: f64 = (click_y - self.canvas_half_height) / scale + com.1;
        // add a random offset to the click position
        let mut rng: ThreadRng = rand::thread_rng();
        let offset: f64 = 0.05 * self.canvas_width / scale;
        let click_x_au: f64 = click_x_au + offset * (rng.gen::<f64>() - 0.5);
        let click_y_au: f64 = click_y_au + offset * (rng.gen::<f64>() - 0.5);
        // add one body at the click position
        self.spawned_x.push(click_x_au);
        self.spawned_y.push(click_y_au);
    }

    pub fn update(&mut self) {
        // if the mouse is clicked, spawn bodies
        if self.clicked {
            self.spawn_body(self.mouse_x, self.mouse_y);
            log(&format!(
                "Spawned body at ({}, {})",
                self.mouse_x, self.mouse_y
            ));
            return;
        }

        // Re-construct the Barnes-Hut tree
        self.bh_tree.construct(&self.x, &self.y, self.num_bodies);
        // Compute the acceleration for each body
        for i in 0..self.num_bodies {
            let (new_ax, new_ay): (f64, f64) = calc_acceleration::add_node_acceleration(
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
            self.vx[i] += self.ax[i] * GRAVITY * DT;
            self.vy[i] += self.ay[i] * GRAVITY * DT;
            self.x[i] += self.vx[i] * DT;
            self.y[i] += self.vy[i] * DT;
        }

        // log enegy
        // log_energy(&self.x, &self.y, &self.vx, &self.vy, self.num_bodies);
    }

    // HELPERS
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
}
