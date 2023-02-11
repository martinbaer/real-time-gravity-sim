use crate::input::Constants;

use super::{Point, Vec2D};

const TREE_GROWTH_INCREMENT: usize = 100;
pub const ROOT_NODE_INDEX: usize = 0;

pub struct Node {
	pub centre_of_mass: Point,
	pub mass: usize,
	pub top_left: usize,
	pub top_right: usize,
	pub bottom_left: usize,
	pub bottom_right: usize,
}
impl Node {
	fn new() -> Node {
		Node {
			centre_of_mass: Point::new(0.0, 0.0),
			mass: 0,
			top_left: 0,
			top_right: 0,
			bottom_left: 0,
			bottom_right: 0,
		}
	}
	fn clone(&self) -> Node {
		Node {
			centre_of_mass: self.centre_of_mass.clone(),
			mass: self.mass,
			top_left: self.top_left,
			top_right: self.top_right,
			bottom_left: self.bottom_left,
			bottom_right: self.bottom_right,
		}
	}
}

struct NodeDesc {
	index: usize,
	centre_x: f64,
	centre_y: f64,
	half_width: f64,
}
impl NodeDesc {
	fn new(half_width: f64) -> NodeDesc {
		NodeDesc {
			index: 0,
			centre_x: 0.0,
			centre_y: 0.0,
			half_width,
		}
	}
	pub fn clone(&self) -> NodeDesc {
		NodeDesc {
			index: self.index,
			centre_x: self.centre_x,
			centre_y: self.centre_y,
			half_width: self.half_width,
		}
	}
}

pub struct Tree {
	pub nodes: Vec<Node>,
	num_nodes: usize,
	vec_size: usize,
	pub root_half_width: f64,
}
impl Tree {
	pub fn new(num_bodies: usize) -> Tree {
		// Create the nodes vector initialized with TREE_GROWTH_INCREMENT new nodes
		let mut nodes: Vec<Node> = Vec::with_capacity(num_bodies);
		for _ in 0..(num_bodies) {
			nodes.push(Node::new());
		}
		Tree {
			nodes,
			num_nodes: 0,
			vec_size: 0,
			root_half_width: 0.0,
		}
	}
}

enum InsertBody {
	New { point_x: f64, point_y: f64 },
	Existing { tree_index: usize }, // This instead stores the index of the body in the tree
}


// Increase the size of the tree
fn grow_tree(tree: &mut Tree) {
	// Increase the size of the tree
	tree.vec_size += TREE_GROWTH_INCREMENT;
	// Allocate space for the new nodes
	tree.nodes.reserve(TREE_GROWTH_INCREMENT);
	// Add the new nodes to the tree
	for _ in 0..TREE_GROWTH_INCREMENT {
		tree.nodes.push(Node::new());
	}
}

// Zeros the values in the node
fn zero_node(tree: &mut Tree, node_index: usize) {
	tree.nodes[node_index].centre_of_mass.x = 0.0;
	tree.nodes[node_index].centre_of_mass.y = 0.0;
	tree.nodes[node_index].mass = 0;
	tree.nodes[node_index].top_left = 0;
	tree.nodes[node_index].top_right = 0;
	tree.nodes[node_index].bottom_left = 0;
	tree.nodes[node_index].bottom_right = 0;
}

// Gets a child node of the parent node to place the given body in
fn get_child(tree: &mut Tree, body: &InsertBody, parent_desc: &NodeDesc) -> NodeDesc {
	let mut child_node_desc: NodeDesc = NodeDesc::new(parent_desc.half_width / 2.0);

	let (body_x, body_y): (f64, f64) = match body {
		InsertBody::New { point_x, point_y } => (*point_x, *point_y),
		InsertBody::Existing { tree_index } => (tree.nodes[*tree_index].centre_of_mass.x, tree.nodes[*tree_index].centre_of_mass.y),
	};

	// Check if the particle is in the top left quadrant
	if body_x <= parent_desc.centre_x && body_y >= parent_desc.centre_y {
		child_node_desc.centre_x = parent_desc.centre_x - child_node_desc.half_width;
		child_node_desc.centre_y = parent_desc.centre_y + child_node_desc.half_width;
		// Check if the top left quadrant does not exists
		if tree.nodes[parent_desc.index].top_left == 0 {
			// Create the top left quadrant
			// Get the next available index
			child_node_desc.index = tree.num_nodes;
			tree.num_nodes += 1;
			// Reallocate the tree to fit the new node
			if tree.num_nodes > tree.vec_size {
				grow_tree(tree);
			}
			// Tell the parent node about the child node
			tree.nodes[parent_desc.index].top_left = child_node_desc.index;
			// Initialize the child node
			zero_node(tree, child_node_desc.index);
		} else {
			// The top left quadrant already exists
			child_node_desc.index = tree.nodes[parent_desc.index].top_left;
		}
	}
	// Check if the particle is in the top right quadrant
	else if body_x >= parent_desc.centre_x && body_y >= parent_desc.centre_y {
		child_node_desc.centre_x = parent_desc.centre_x + child_node_desc.half_width;
		child_node_desc.centre_y = parent_desc.centre_y + child_node_desc.half_width;
		// Check if the top right quadrant does not exists
		if tree.nodes[parent_desc.index].top_right == 0 {
			// Create the top right quadrant
			// Get the next available index
			child_node_desc.index = tree.num_nodes;
			tree.num_nodes += 1;
			// Reallocate the tree to fit the new node
			if tree.num_nodes > tree.vec_size {
				grow_tree(tree);
			}
			// Tell the parent node about the child node
			tree.nodes[parent_desc.index].top_right = child_node_desc.index;
			// Initialise the child node
			zero_node(tree, child_node_desc.index);
		} else {
			// The top right quadrant already exists
			child_node_desc.index = tree.nodes[parent_desc.index].top_right;
		}
	}
	// Check if the particle is in the bottom left quadrant
	else if body_x <= parent_desc.centre_x && body_y <= parent_desc.centre_y {
		child_node_desc.centre_x = parent_desc.centre_x - child_node_desc.half_width;
		child_node_desc.centre_y = parent_desc.centre_y - child_node_desc.half_width;
		// Check if the bottom left quadrant does not exists
		if tree.nodes[parent_desc.index].bottom_left == 0 {
			// Create the bottom left quadrant
			// Get the next available index
			child_node_desc.index = tree.num_nodes;
			tree.num_nodes += 1;
			// Reallocate the tree to fit the new node
			if tree.num_nodes > tree.vec_size {
				grow_tree(tree);
			}
			// Tell the parent node about the child node
			tree.nodes[parent_desc.index].bottom_left = child_node_desc.index;
			// Initialise the child node
			zero_node(tree, child_node_desc.index);
		} else {
			// The bottom left quadrant already exists
			child_node_desc.index = tree.nodes[parent_desc.index].bottom_left;
		}
	}
	// Check if the particle is in the bottom right quadrant
	else {
		child_node_desc.centre_x = parent_desc.centre_x + child_node_desc.half_width;
		child_node_desc.centre_y = parent_desc.centre_y - child_node_desc.half_width;
		// Check if the bottom right quadrant does not exists
		if tree.nodes[parent_desc.index].bottom_right == 0 {
			// Create the bottom right quadrant
			// Get the next available index
			child_node_desc.index = tree.num_nodes;
			tree.num_nodes += 1;
			// Reallocate the tree to fit the new node
			if tree.num_nodes > tree.vec_size {
				grow_tree(tree);
			}
			// Tell the parent node about the child node
			tree.nodes[parent_desc.index].bottom_right = child_node_desc.index;
			// Initialise the child node
			zero_node(tree, child_node_desc.index);
		} else {
			// The bottom right quadrant already exists
			child_node_desc.index = tree.nodes[parent_desc.index].bottom_right;
		}
	}
	child_node_desc
}


// Insert a body into the Barnes-Hut tree
fn insert(tree: &mut Tree, body: InsertBody, node_desc: NodeDesc) {
	// Get the x and y coordinates of the body
	let (body_x, body_y): (f64, f64) = match body {
		InsertBody::New { point_x, point_y } => (point_x, point_y),
		InsertBody::Existing { tree_index } => (tree.nodes[tree_index].centre_of_mass.x, tree.nodes[tree_index].centre_of_mass.y),
	};

	// Clone the node
	let node: &Node = &tree.nodes[node_desc.index];
	// Check if the node is a leaf
	if node.top_left == 0 && node.top_right == 0 && node.bottom_left == 0 && node.bottom_right == 0 {
		// Node is a leaf
		if node.mass == 0 {
			// Node is an empty leaf: add the particle
			tree.nodes[node_desc.index].mass = 1;
			tree.nodes[node_desc.index].centre_of_mass.x = body_x;
			tree.nodes[node_desc.index].centre_of_mass.y = body_y;
		}
		else {
			// Node is an occupied leaf: split the node and add the existing and new particles
			// Add the existing particle to child of the tree
			// let child_desc: NodeDesc = get_child(tree,  &node.centre_of_mass, &node_desc);
			let quadrant_as_body = InsertBody::Existing{ tree_index: node_desc.index };
			let child_desc: NodeDesc = get_child(tree, &quadrant_as_body, &node_desc);
			insert(tree, quadrant_as_body, child_desc);
			// Re-try adding new particle to the tree
			insert(tree, body, node_desc);
		}
	}
	else {
		// Node is not a leaf: update the mass and centre of mass of the node
		tree.nodes[node_desc.index].centre_of_mass.x = (tree.nodes[node_desc.index].centre_of_mass.x * (tree.nodes[node_desc.index].mass as f64) + body_x) / (tree.nodes[node_desc.index].mass + 1) as f64;
		tree.nodes[node_desc.index].centre_of_mass.y = (tree.nodes[node_desc.index].centre_of_mass.y * (tree.nodes[node_desc.index].mass as f64) + body_y) / (tree.nodes[node_desc.index].mass + 1) as f64;
		tree.nodes[node_desc.index].mass += 1;
		// Add the particle to the appropriate child
		let child_desc: NodeDesc = get_child(tree, &body, &node_desc);
		insert(tree, body, child_desc);
	}
}


// Construct the Barnes-Hut tree by adding all the bodies to the tree
pub fn construct_tree(tree: &mut Tree, bodies: &Vec2D, constants: &Constants) {
	// Initialize the root node
	zero_node(tree, ROOT_NODE_INDEX);
	// Calculate the half width of the root node
	let mut min: Point = Point::new(f64::MAX, f64::MAX);
	let mut max: Point = Point::new(f64::MIN, f64::MIN);
	for i in 0..constants.num_bodies {
		if bodies.x[i] < min.x {
			min.x = bodies.x[i];
		}
		if bodies.y[i] < min.y {
			min.y = bodies.y[i];
		}
		if bodies.x[i] > max.x {
			max.x = bodies.x[i];
		}
		if bodies.y[i] > max.y {
			max.y = bodies.y[i];
		}
	}
	// max of 2 floats
	let half_width: f64 = if max.x - min.x > max.y - min.y { max.x - min.x } else { max.y - min.y } / 2.0;
	tree.root_half_width = half_width;
	// Initialize the root node descriptor
	let root_node_desc: NodeDesc = NodeDesc {
		index: ROOT_NODE_INDEX,
		centre_x: 0.0,
		centre_y: 0.0,
		half_width,
	};
	for i in 0..constants.num_bodies {
		insert(tree, InsertBody::New { point_x: bodies.x[i], point_y: bodies.y[i] }, root_node_desc.clone());
	}
}

// Print the Barnes-Hut tree to the console
pub fn print_tree(tree: &Tree, depth: usize, node_index: usize) {
	if node_index == 0 && depth != 0 {
		return;
	}
	for _ in 0..depth {
		print!("  ");
	}
	let node: &Node = &tree.nodes[node_index];
	println!("Node {}: mass {}, centre of mass ({}, {})", node_index, node.mass, node.centre_of_mass.x, node.centre_of_mass.y);
	if node.top_left != 0 {
		print_tree(tree, depth + 1, node.top_left);
	}
	if node.top_right != 0 {
		print_tree(tree, depth + 1, node.top_right);
	}
	if node.bottom_left != 0 {
		print_tree(tree, depth + 1, node.bottom_left);
	}
	if node.bottom_right != 0 {
		print_tree(tree, depth + 1, node.bottom_right);
	}
}