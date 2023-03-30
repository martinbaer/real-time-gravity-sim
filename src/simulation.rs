use rand::{rngs::ThreadRng, Rng};

mod bh_tree;
mod calc_acceleration;
mod energy_conservation;
mod spawner;

use crate::constants::{
    BODY_DRAW_SIZE, BODY_DRAW_SIZE_MOBILE, DT, GRAVITY, ROOT_NODE_INDEX, SPAWNED_BODY_SPEED,
    SPAWNED_BODY_SPEED_MOBILE, SPAWN_BODY_COLOR, SPAWN_BODY_DRAW_SIZE, SPAWN_BODY_DRAW_SIZE_MOBILE,
    SPAWN_RANDOM_OFFSET, SPEED_LIMIT, SPEED_LIMIT_SQ, START_BOX_SIZE, STAR_COLOURS,
    STAR_COLOURS_LEN,
};
use crate::{draw_body, log, log_u32};

use self::bh_tree::Tree;
use self::energy_conservation::log_energy;
use self::spawner::Spawner;

// The bodies struct is a Struct of Arrays (SoA) implementation of the bodies
pub struct Simulation {
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
    spawner: Spawner,
    clicked: bool,
    pub com: (f64, f64),
    pub scale: f64,
}
impl Simulation {
    pub fn new_empty() -> Simulation {
        Simulation {
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
            spawner: Spawner::new_empty(),
            clicked: false,
            com: (0.0, 0.0),
            scale: 1.0,
        }
    }
    pub fn create(&mut self, num: usize, canvas_width: f64, canvas_height: f64, is_mobile: bool) {
        self.is_mobile = is_mobile;
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        self.canvas_half_width = canvas_width / 2.0;
        self.canvas_half_height = canvas_height / 2.0;
        self.num_bodies = num;
        self.spawner.create(is_mobile, canvas_width, canvas_height);
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
            self.x.push(rng.gen_range(-START_BOX_SIZE..START_BOX_SIZE));
            self.y.push(rng.gen_range(-START_BOX_SIZE..START_BOX_SIZE));

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
    pub fn on_click(&mut self, x: f64, y: f64) {
        self.clicked = true;
        self.spawner.on_click(x, y);
    }
    pub fn off_click(&mut self, x: f64, y: f64) {
        self.clicked = false;
        self.spawner.off_click(
            x,
            y,
            &mut self.x,
            &mut self.y,
            &mut self.vx,
            &mut self.vy,
            &mut self.ax,
            &mut self.ay,
            &mut self.com_distances,
            &mut self.num_bodies,
            &mut self.bh_tree,
        );
    }

    pub fn draw(&mut self) {
        self.com = self.get_com();
        let percentile: f64 = self.get_99th_percentile(self.com);
        self.scale = self.canvas_width / (2.0 * percentile);
        for i in 0..self.num_bodies {
            // calculate the canvas position of the body such that the centre of mass is at the centre of the canvas (canvas_half_width, canvas_half_height) and 99% of the bodies are inside the canvas
            let canvas_x: f64 = (self.x[i] - self.com.0) * self.scale + self.canvas_half_width;
            let canvas_y: f64 = (self.y[i] - self.com.1) * self.scale + self.canvas_half_height;
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

        self.spawner.draw_spawned_bodies(self.com, self.scale);

        // print percentile * 2
        // log(&format!("Width (AU): {}", percentile * 2.0));
    }

    pub fn update(&mut self) {
        // if the mouse is clicked, spawn bodies
        if self.clicked {
            self.spawner.spawn_body(self.com, self.scale);
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
            // check if exceeding max velocity
            if self.vx[i] * self.vx[i] + self.vy[i] * self.vy[i] > SPEED_LIMIT_SQ {
                log("speed limit hit");
                let speed: f64 = (self.vx[i] * self.vx[i] + self.vy[i] * self.vy[i]).sqrt();
                self.vx[i] *= SPEED_LIMIT / speed;
                self.vy[i] *= SPEED_LIMIT / speed;
            }
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
        self.com_distances[(0.98 * self.num_bodies as f64) as usize]
    }
}
