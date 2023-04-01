use std::thread::Thread;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    constants::{
        BODIES_PER_SPAWN, SPAWN_BODY_COLOR, SPAWN_BODY_DRAW_SIZE, SPAWN_BODY_DRAW_SIZE_MOBILE,
    },
    draw_body,
};

use super::bh_tree::Tree;

pub struct Spawner {
    mouse_x: f64,
    mouse_y: f64,
    spawned_x: Vec<f64>,
    spawned_y: Vec<f64>,
    is_mobile: bool,
    canvas_width: f64,
    canvas_height: f64,
    canvas_half_width: f64,
    canvas_half_height: f64,
    pub spawn_radius: f64,
    pub spawn_speed: f64,
}
impl Spawner {
    pub fn new_empty() -> Spawner {
        Spawner {
            mouse_x: 0.0,
            mouse_y: 0.0,
            spawned_x: Vec::new(),
            spawned_y: Vec::new(),
            is_mobile: false,
            canvas_width: 0.0,
            canvas_height: 0.0,
            canvas_half_width: 0.0,
            canvas_half_height: 0.0,
            spawn_radius: 0.0,
            spawn_speed: 0.0,
        }
    }
    pub fn create(&mut self, is_mobile: bool, canvas_width: f64, canvas_height: f64) {
        self.is_mobile = is_mobile;
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        self.canvas_half_width = canvas_width / 2.0;
        self.canvas_half_height = canvas_height / 2.0;
    }
    // spawn bodies randomly around the click position
    pub fn spawn_body(&mut self, com: (f64, f64), scale: f64) {
        // convert click position from canvas to AU
        let x: f64 = (self.mouse_x - self.canvas_half_width) / scale + com.0;
        let y: f64 = (self.mouse_y - self.canvas_half_height) / scale + com.1;
        for _ in 0..BODIES_PER_SPAWN {
            // add a random offset to the click position
            let offset: f64 = self.spawn_radius;
            let mut rng: ThreadRng = rand::thread_rng();
            let click_x_au: f64 = x + offset * (rng.gen::<f64>() - 0.5);
            let click_y_au: f64 = y + offset * (rng.gen::<f64>() - 0.5);
            // add one body at the click position
            self.spawned_x.push(click_x_au);
            self.spawned_y.push(click_y_au);
        }
    }
    pub fn update_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_x = x;
        self.mouse_y = y;
    }
    pub fn add_spawned_bodies_to_simulation(
        &mut self,
        mouse_x: f64,
        mouse_y: f64,
        sim_x: &mut Vec<f64>,
        sim_y: &mut Vec<f64>,
        sim_vx: &mut Vec<f64>,
        sim_vy: &mut Vec<f64>,
        sim_ax: &mut Vec<f64>,
        sim_ay: &mut Vec<f64>,
        sim_com_distances: &mut Vec<f64>,
        sim_num_bodies: &mut usize,
        sim_bh_tree: &mut Tree,
    ) {
        // calculate the velocity of the spawned bodies
        let dx: f64 = mouse_x - self.mouse_x;
        let dy: f64 = mouse_y - self.mouse_y;
        let vx: f64 = dx * self.spawn_speed;
        let vy: f64 = dy * self.spawn_speed;

        // add all spawned bodies to the system
        for i in 0..self.spawned_x.len() {
            sim_x.push(self.spawned_x[i]);
            sim_y.push(self.spawned_y[i]);
            sim_vx.push(vx);
            sim_vy.push(vy);
            sim_ax.push(0.0);
            sim_ay.push(0.0);
            sim_com_distances.push(0.0);
        }
        self.spawned_x.clear();
        self.spawned_y.clear();
        *sim_num_bodies = sim_x.len();
        *sim_bh_tree = Tree::new();
    }

    pub fn draw_spawned_bodies(&self, com: (f64, f64), scale: f64) {
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
    }
}
