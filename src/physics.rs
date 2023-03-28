use rand::{rngs::ThreadRng, Rng};

mod bh_tree;

use crate::constants::{BODY_DRAW_SIZE, STAR_COLOURS, STAR_COLOURS_LEN};
use crate::{draw_body, log, log_u32};

use self::bh_tree::Tree;

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
    pub bh_tree: bh_tree::Tree,
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
            bh_tree: Tree::new(),
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
        for _ in 0..num {
            let mut rng: ThreadRng = rand::thread_rng();
            self.x.push(self.canvas_width * rng.gen::<f64>());
            self.y.push(self.canvas_height * rng.gen::<f64>());
            self.vx.push(4.0 * rng.gen::<f64>() - 2.0);
            self.vy.push(4.0 * rng.gen::<f64>() - 2.0);
            self.ax.push(0.0);
            self.ay.push(0.0);
        }
        // Set the half-width of the root node to the largest dimension of the system
        self.bh_tree.root_half_width = self.canvas_width.max(self.canvas_height) / 2.0;
        // Set the centre of the root node to the centre of the system
        self.bh_tree.root_centre = (self.canvas_width / 2.0, self.canvas_height / 2.0);
    }

    pub fn draw(&self) {
        // draw the bodies with color rotating through STAR_COLOURS
        for i in 0..self.num_bodies {
            let color: &str = STAR_COLOURS[i % STAR_COLOURS_LEN];
            draw_body(self.x[i], self.y[i], color, BODY_DRAW_SIZE);
        }
    }

    pub fn update(&mut self) {
        // print "update" to the console
        log_u32(self.num_bodies as u32);
        log("updating");

        // Re-construct the Barnes-Hut tree
        self.bh_tree.construct(&self.x, &self.y, self.num_bodies);

        log("successfully constructed tree");
        log(self.bh_tree.print(0, 0).as_str());
        self.bh_tree.construct(&self.x, &self.y, self.num_bodies);

        log("successfully constructed tree");
        log(self.bh_tree.print(0, 0).as_str());
        // close program (wasm)
        std::process::exit(0);
    }
}

// // Takes a mutable reference to a vector of position points and the input constants and moves each point
// pub fn step(
//     x: &mut Vec<f64>,
//     y: &mut Vec<f64>,
//     vx: &mut Vec<f64>,
//     vy: &mut Vec<f64>,
//     ax: &mut Vec<f64>,
//     ay: &mut Vec<f64>,
//     bh_tree: &mut Tree,
//     num_bodies: usize,
// ) {
//     // Construct the Barnes-Hut tree
//     bh_tree::construct_tree(bh_tree, x, y, num_bodies);
//     // bh_tree::print_tree(bh_tree, 0, 0);
//     // // close program
//     // std::process::exit(0);
//     // SIMD
//     for i in 0..constants.num_bodies {
//         // Zero the acceleration
//         accelerations.x[i] = 0.0;
//         accelerations.y[i] = 0.0;
//     }

//     // not SIMD
//     for i in 0..constants.num_bodies {
//         // Calculate the acceleration
//         let (acc_x, acc_y) = add_node_acceleration(
//             (&positions.x[i], &positions.y[i]),
//             bh_tree,
//             bh_tree::ROOT_NODE_INDEX,
//             bh_tree.root_half_width,
//             constants,
//         );
//         accelerations.x[i] = acc_x;
//         accelerations.y[i] = acc_y;
//     }

//     // SIMD
//     for i in 0..constants.num_bodies {
//         // Multiply the acceleration by gravity
//         accelerations.x[i] *= constants.gravity;
//         accelerations.y[i] *= constants.gravity;
//     }
//     // SIMD
//     for i in 0..constants.num_bodies {
//         // Update the velocity
//         velocities.x[i] += accelerations.x[i] * constants.delta_t;
//         velocities.y[i] += accelerations.y[i] * constants.delta_t;
//     }
//     // SIMD
//     for i in 0..constants.num_bodies {
//         // Update the position
//         positions.x[i] += velocities.x[i] * constants.delta_t;
//         positions.y[i] += velocities.y[i] * constants.delta_t;
//     }
// }
