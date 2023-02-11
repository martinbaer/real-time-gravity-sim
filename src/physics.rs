
pub mod logger;
pub mod bh_tree;

use crate::input::Constants;

use self::bh_tree::Tree;

pub struct Point {
	pub x: f64,
	pub y: f64,
}
impl Point {
	pub fn new(x: f64, y: f64) -> Point {
		Point { x, y }
	}
	pub fn clone(&self) -> Point {
		Point { x: self.x, y: self.y }
	}
}

pub struct Vec2D {
	pub x: Vec<f64>,
	pub y: Vec<f64>,
}
impl Vec2D {
	pub fn new(num_bodies: usize) -> Vec2D {
		Vec2D {
			x: vec![0.0; num_bodies],
			y: vec![0.0; num_bodies],
		}
	}
}

fn add_node_acceleration(acceleration: &mut (f64, f64), body: (&f64, &f64), bh_tree: &Tree, node_index: usize, s: f64, constants: &Constants) {
	let (acc_x, acc_y): &mut (f64, f64) = acceleration;
	let (body_x, body_y): (&f64, &f64) = body;
	// Calculate the distance between the particle and the node
	let dx: f64 = bh_tree.nodes[node_index].centre_of_mass.x - body_x;
	let dy: f64 = bh_tree.nodes[node_index].centre_of_mass.y - body_y;
	let d: f64 = (dx * dx + dy * dy).sqrt();
	// If the node is a leaf, add the acceleration
	if bh_tree.nodes[node_index].mass == 1 {
		// Calculate and add the acceleration (mass is 1)
		*acc_x += dx / (d * d * d + constants.softening);
		*acc_y += dy / (d * d * d + constants.softening);
	}
	// If the node is not a leaf, check if the node is far enough to take its centre of mass
	else {
		// Check the s/d ratio for the node
		if s / d < constants.theta {
			// Calculate and add the acceleration (mass is >1)
			*acc_x += bh_tree.nodes[node_index].mass as f64 * dx / (d * d * d + constants.softening);
			*acc_y += bh_tree.nodes[node_index].mass as f64 * dy / (d * d * d + constants.softening);
		}
		else {
			// Recursively calculate the acceleration
			let new_s = s / 2.0;
			if bh_tree.nodes[node_index].bottom_left != 0 {
				add_node_acceleration(acceleration, body, bh_tree, bh_tree.nodes[node_index].bottom_left, new_s, constants);
			}
			if bh_tree.nodes[node_index].bottom_right != 0 {
				add_node_acceleration(acceleration, body, bh_tree, bh_tree.nodes[node_index].bottom_right, new_s, constants);
			}
			if bh_tree.nodes[node_index].top_left != 0 {
				add_node_acceleration(acceleration, body, bh_tree, bh_tree.nodes[node_index].top_left, new_s, constants);
			}
			if bh_tree.nodes[node_index].top_right != 0 {
				add_node_acceleration(acceleration, body, bh_tree, bh_tree.nodes[node_index].top_right, new_s, constants);
			}
		}
	}
}

// Takes a mutable reference to a vector of position points and the input constants and moves each point
pub fn step(positions: &mut Vec2D, velocities: &mut Vec2D, accelerations: &mut Vec2D, bh_tree: &mut Tree, constants: &Constants) {
	// Construct the Barnes-Hut tree
	bh_tree::construct_tree(bh_tree, positions, constants);
	bh_tree::print_tree(bh_tree, 0, 0);
	// close program
	std::process::exit(0);
	// SIMD
	for i in 0..constants.num_bodies {
		// Zero the acceleration
		accelerations.x[i] = 0.0;
		accelerations.y[i] = 0.0;
	}
	// not SIMD
	for i in 0..constants.num_bodies {
		// Calculate the acceleration
		add_node_acceleration(&mut (accelerations.x[i], accelerations.y[i]), (&positions.x[i], &positions.y[i]), bh_tree, bh_tree::ROOT_NODE_INDEX, bh_tree.root_half_width, constants);
	}
	// SIMD
	for i in 0..constants.num_bodies {
		// Multiply the acceleration by gravity
		accelerations.x[i] *= constants.gravity;
		accelerations.y[i] *= constants.gravity;
	}
	// SIMD
	for i in 0..constants.num_bodies {
		// Update the velocity
		velocities.x[i] += accelerations.x[i] * constants.delta_t;
		velocities.y[i] += accelerations.y[i] * constants.delta_t;
	}
	// SIMD
	for i in 0..constants.num_bodies {
		// Update the position
		positions.x[i] += velocities.x[i] * constants.delta_t;
		positions.y[i] += velocities.y[i] * constants.delta_t;
	}
}