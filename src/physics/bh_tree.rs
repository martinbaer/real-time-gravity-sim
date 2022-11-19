use super::Point;

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
}

pub struct Tree {
	pub nodes: Vec<Node>,
	num_nodes: usize,
	vec_size: usize,
	pub root_half_width: f64,
}
impl Tree {
	pub fn new() -> Tree {
		// Create the nodes vector initialized with TREE_GROWTH_INCREMENT new nodes
		let mut nodes: Vec<Node> = Vec::with_capacity(TREE_GROWTH_INCREMENT);
		for _ in 0..TREE_GROWTH_INCREMENT {
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
fn get_child(tree: &mut Tree, body: Point, parent_desc: &NodeDesc) -> NodeDesc {
	let mut child_node_desc: NodeDesc = NodeDesc::new(parent_desc.half_width / 2.0);
	// Check if the particle is in the top left quadrant
	if body.x <= parent_desc.centre_x && body.y >= parent_desc.centre_y {
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
			// Initialise the child node
			zero_node(tree, child_node_desc.index);
		} else {
			// The top left quadrant already exists
			child_node_desc.index = tree.nodes[parent_desc.index].top_left;
		}
	}
	// Check if the particle is in the top right quadrant
	else if body.x >= parent_desc.centre_x && body.y >= parent_desc.centre_y {
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
	else if body.x <= parent_desc.centre_x && body.y <= parent_desc.centre_y {
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
fn insert(tree: &mut Tree, body: &Point, node_desc: &NodeDesc) {
	// Check if the node is a leaf
	if tree.nodes[node_desc.index].top_left == 0 || tree.nodes[node_desc.index].top_right == 0 || !tree.nodes[node_desc.index].bottom_left == 0 || !tree.nodes[node_desc.index].bottom_right == 0 {
		// Node is a leaf
		if tree.nodes[node_desc.index].mass == 1 {
			// Node is an empty leaf: add the particle
			tree.nodes[node_desc.index].mass = 1;
			tree.nodes[node_desc.index].centre_of_mass.x = body.x;
			tree.nodes[node_desc.index].centre_of_mass.y = body.y;
		}
		else {
			// Node is an occupied leaf: split the node and add the existing and new particles
			// Add the existing particle to child of the tree
			// let child_desc: NodeDesc = get_child(tree,  &node.centre_of_mass, &node_desc);
			let child_desc: NodeDesc = get_child(tree,  tree.nodes[node_desc.index].centre_of_mass.clone(), &node_desc);
			insert(tree, &Point::new(tree.nodes[node_desc.index].centre_of_mass.x, tree.nodes[node_desc.index].centre_of_mass.y), &child_desc);
			// Re-try adding new particle to the tree
			insert(tree, body, node_desc);
		}
	}
	else {
		// Node is not a leaf: update the mass and centre of mass of the node
		tree.nodes[node_desc.index].centre_of_mass.x = (tree.nodes[node_desc.index].centre_of_mass.x * (tree.nodes[node_desc.index].mass as f64) + body.x) / (tree.nodes[node_desc.index].mass + 1) as f64;
		tree.nodes[node_desc.index].centre_of_mass.y = (tree.nodes[node_desc.index].centre_of_mass.y * (tree.nodes[node_desc.index].mass as f64) + body.y) / (tree.nodes[node_desc.index].mass + 1) as f64;
		tree.nodes[node_desc.index].mass += 1;
		// Add the particle to the appropriate child
		let child_desc: NodeDesc = get_child(tree, body.clone(), &node_desc);
		insert(tree, body, &child_desc);
	}
}


// Construct the Barnes-Hut tree by adding all the bodies to the tree
pub fn construct_tree(tree: &mut Tree, bodies: &Vec<Point>) {
	// Initialize the root node
	zero_node(tree, ROOT_NODE_INDEX);
	// Calculate the half width of the root node
	let mut min: Point = Point::new(f64::MAX, f64::MAX);
	let mut max: Point = Point::new(f64::MIN, f64::MIN);
	for body in bodies {
		if body.x < min.x {
			min.x = body.x;
		}
		if body.y < min.y {
			min.y = body.y;
		}
		if body.x > max.x {
			max.x = body.x;
		}
		if body.y > max.y {
			max.y = body.y;
		}
	}
	let half_width: f64 = (max.x - min.x).max(max.y - min.y) / 2.0;
	tree.root_half_width = half_width;
	// Initialise the root node descriptor
	let root_node_desc: NodeDesc = NodeDesc {
		index: ROOT_NODE_INDEX,
		centre_x: 0.0,
		centre_y: 0.0,
		half_width,
	};
	for body in bodies {
		insert(tree, body, &root_node_desc);
	}
}