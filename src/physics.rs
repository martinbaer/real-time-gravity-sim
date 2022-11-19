
pub mod logger;
pub mod bh_tree;

use crate::input::Constants;

use self::bh_tree::Tree;

// 2-dimensional point
pub struct Point {
	pub x: f64,
	pub y: f64,
}
impl Point {
	// Returns the distance between two points
	pub fn new(x: f64, y: f64) -> Point {
		Point {
			x,
			y,
		}
	}

fn clone(&self) -> Point {
	Point {
		x: self.x,
		y: self.y,
	}
}
}

fn add_node_acceleration(acceleration: &mut Point, body: &Point, bh_tree: &Tree, node_index: usize, s: f64, constants: &Constants) {
	// Calculate the distance between the particle and the node
	let dx = bh_tree.nodes[node_index].centre_of_mass.x - body.x;
	let dy = bh_tree.nodes[node_index].centre_of_mass.y - body.y;
	let d: f64 = (dx * dx + dy * dy).sqrt();
	// If the node is a leaf, add the acceleration
	if bh_tree.nodes[node_index].mass == 1 {
		// Calculate and add the acceleration (mass is 1)
		acceleration.x += dx / (d * d * d + constants.softening);
		acceleration.y += dy / (d * d * d + constants.softening);
	}
	// If the node is not a leaf, check if the node is far enough to take its centre of mass
	else {
		// Check the s/d ratio for the node
		if s / d < constants.theta {
			// Calculate and add the acceleration (mass is >1)
			acceleration.x += bh_tree.nodes[node_index].mass as f64 * dx / (d * d * d + constants.softening);
			acceleration.y += bh_tree.nodes[node_index].mass as f64 * dy / (d * d * d + constants.softening);
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

// Returns the acceleration on a given point using the Barnes-Hut algorithm
fn calculate_acceleration(body: &Point, bh_tree: &Tree, constants: &Constants) -> Point {
	// Initialize the acceleration
	let mut acceleration: Point = Point::new(0.0, 0.0);
	// Calculate the acceleration
	add_node_acceleration(&mut acceleration, body, bh_tree, bh_tree::ROOT_NODE_INDEX, bh_tree.root_half_width, constants);
	// Multiply by the gravitational constant
	acceleration.x *= constants.gravity;
	acceleration.y *= constants.gravity;
	// Return the acceleration
	acceleration
}

// Takes a mutable reference to a vector of position points and the input constants and moves each point
pub fn step(positions: &mut Vec<Point>, velocities: &mut Vec<Point>, bh_tree: &mut Tree, constants: &Constants) {
	// Construct the Barnes-Hut tree
	bh_tree::construct_tree(bh_tree, positions);
	for i in 0..positions.len() {
		// Calculate the acceleration
		let acceleration: Point = calculate_acceleration(&positions[i], bh_tree, constants);
		// Update the velocity
		velocities[i].x += acceleration.x * constants.delta_t;
		velocities[i].y += acceleration.y * constants.delta_t;
		// Update the position
		positions[i].x += velocities[i].x * constants.delta_t;
		positions[i].y += velocities[i].y * constants.delta_t;
	}
}