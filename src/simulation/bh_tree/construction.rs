use crate::constants::{ROOT_NODE_INDEX, TREE_GROWTH_INCREMENT};

use super::{nodes::NodeDesc, Node, Tree};

struct Insert {
    body_x: f64,
    body_y: f64,
    target_node: NodeDesc,
}
impl Insert {
    fn clone(&self) -> Insert {
        Insert {
            body_x: self.body_x,
            body_y: self.body_y,
            target_node: self.target_node.clone(),
        }
    }
}

impl Tree {
    fn grow(&mut self) {
        // Increase the size of the tree
        self.vec_size += TREE_GROWTH_INCREMENT;
        // Allocate space for the new nodes
        self.nodes.reserve(TREE_GROWTH_INCREMENT);
        // Add the new nodes to the tree
        for _ in 0..TREE_GROWTH_INCREMENT {
            self.nodes.push(Node::new());
        }
    }
    fn zero_node(&mut self, node_index: usize) {
        self.nodes[node_index].centre_of_mass = (0.0, 0.0);
        self.nodes[node_index].mass = 0;
        self.nodes[node_index].top_left = 0;
        self.nodes[node_index].top_right = 0;
        self.nodes[node_index].bottom_left = 0;
        self.nodes[node_index].bottom_right = 0;
    }
    // Gets a child node of the parent node to place the given body in
    fn get_child_node(&mut self, body_x: f64, body_y: f64, parent_desc: &NodeDesc) -> NodeDesc {
        let mut child_node_desc: NodeDesc = NodeDesc::new(parent_desc.half_width / 2.0);

        // Check if the particle is in the top left quadrant
        if body_x <= parent_desc.centre_x && body_y >= parent_desc.centre_y {
            child_node_desc.centre_x = parent_desc.centre_x - child_node_desc.half_width;
            child_node_desc.centre_y = parent_desc.centre_y + child_node_desc.half_width;
            // Check if the top left quadrant does not exists
            if self.nodes[parent_desc.index].top_left == 0 {
                // Create the top left quadrant
                // Get the next available index
                child_node_desc.index = self.num_nodes;
                self.num_nodes += 1;
                // Reallocate the tree to fit the new node
                if self.num_nodes > self.vec_size {
                    self.grow();
                }
                // Tell the parent node about the child node
                self.nodes[parent_desc.index].top_left = child_node_desc.index;
                // Initialize the child node unless it is the first run through
                self.zero_node(child_node_desc.index);
            } else {
                // The top left quadrant already exists
                child_node_desc.index = self.nodes[parent_desc.index].top_left;
            }
        }
        // Check if the particle is in the top right quadrant
        else if body_x >= parent_desc.centre_x && body_y >= parent_desc.centre_y {
            child_node_desc.centre_x = parent_desc.centre_x + child_node_desc.half_width;
            child_node_desc.centre_y = parent_desc.centre_y + child_node_desc.half_width;
            // Check if the top right quadrant does not exists
            if self.nodes[parent_desc.index].top_right == 0 {
                // Create the top right quadrant
                // Get the next available index
                child_node_desc.index = self.num_nodes;
                self.num_nodes += 1;
                // Reallocate the tree to fit the new node
                if self.num_nodes > self.vec_size {
                    self.grow();
                }
                // Tell the parent node about the child node
                self.nodes[parent_desc.index].top_right = child_node_desc.index;
                // Initialise the child node
                self.zero_node(child_node_desc.index);
            } else {
                // The top right quadrant already exists
                child_node_desc.index = self.nodes[parent_desc.index].top_right;
            }
        }
        // Check if the particle is in the bottom left quadrant
        else if body_x <= parent_desc.centre_x && body_y <= parent_desc.centre_y {
            child_node_desc.centre_x = parent_desc.centre_x - child_node_desc.half_width;
            child_node_desc.centre_y = parent_desc.centre_y - child_node_desc.half_width;
            // Check if the bottom left quadrant does not exists
            if self.nodes[parent_desc.index].bottom_left == 0 {
                // Create the bottom left quadrant
                // Get the next available index
                child_node_desc.index = self.num_nodes;
                self.num_nodes += 1;
                // Reallocate the tree to fit the new node
                if self.num_nodes > self.vec_size {
                    self.grow();
                }
                // Tell the parent node about the child node
                self.nodes[parent_desc.index].bottom_left = child_node_desc.index;
                // Initialise the child node
                self.zero_node(child_node_desc.index);
            } else {
                // The bottom left quadrant already exists
                child_node_desc.index = self.nodes[parent_desc.index].bottom_left;
            }
        }
        // Check if the particle is in the bottom right quadrant
        else {
            child_node_desc.centre_x = parent_desc.centre_x + child_node_desc.half_width;
            child_node_desc.centre_y = parent_desc.centre_y - child_node_desc.half_width;
            // Check if the bottom right quadrant does not exists
            if self.nodes[parent_desc.index].bottom_right == 0 {
                // Create the bottom right quadrant
                // Get the next available index
                child_node_desc.index = self.num_nodes;
                self.num_nodes += 1;
                // Reallocate the tree to fit the new node
                if self.num_nodes > self.vec_size {
                    self.grow();
                }
                // Tell the parent node about the child node
                self.nodes[parent_desc.index].bottom_right = child_node_desc.index;
                // Initialise the child node
                self.zero_node(child_node_desc.index);
            } else {
                // The bottom right quadrant already exists
                child_node_desc.index = self.nodes[parent_desc.index].bottom_right;
            }
        }
        child_node_desc
    }
    pub fn construct(&mut self, x: &Vec<f64>, y: &Vec<f64>, num_bodies: usize) {
        // Initialise the root node
        self.zero_node(ROOT_NODE_INDEX);
        self.num_nodes = 1;
        // Initialise root node description
        let root_node_desc: NodeDesc = NodeDesc {
            index: ROOT_NODE_INDEX,
            centre_x: self.root_centre.0,
            centre_y: self.root_centre.1,
            half_width: self.root_half_width,
        };
        // Initialise insert stack
        let mut insert_stack: Vec<Insert> = Vec::with_capacity(num_bodies); // TODO: avoid reallocation
        for i in 0..num_bodies {
            // Initialise insert
            let insert: Insert = Insert {
                body_x: x[i],
                body_y: y[i],
                target_node: root_node_desc.clone(),
            };
            // Push insert onto stack
            insert_stack.push(insert);
        }
        // Process insert stack
        while !insert_stack.is_empty() {
            // Pop insert from stack
            let insert: Insert = insert_stack.pop().unwrap();
            // Get the x and y coordinates of the body
            let (body_x, body_y): (f64, f64) = (insert.body_x, insert.body_y);
            // Create reference to the node
            let target_node: &Node = &self.nodes[insert.target_node.index];
            // Check if the node is a leaf
            if target_node.is_leaf() {
                // Node is a leaf
                if target_node.mass == 0 {
                    // Node is an empty leaf: add the particle
                    self.nodes[insert.target_node.index].mass = 1;
                    self.nodes[insert.target_node.index].centre_of_mass = (body_x, body_y);
                } else {
                    // Node is an occupied leaf: split the node and add the existing and new particles
                    // Add the existing particle to child of the tree
                    let quadrant_body_x: f64 =
                        self.nodes[insert.target_node.index].centre_of_mass.0;
                    let quadrant_body_y: f64 =
                        self.nodes[insert.target_node.index].centre_of_mass.1;
                    let child_desc: NodeDesc =
                        self.get_child_node(quadrant_body_x, quadrant_body_y, &insert.target_node);
                    let existing_insert_as_child: Insert = Insert {
                        body_x: quadrant_body_x,
                        body_y: quadrant_body_y,
                        target_node: child_desc,
                    };
                    insert_stack.push(existing_insert_as_child);
                    // Re-try adding new particle to the tree
                    insert_stack.push(insert.clone());
                }
            } else {
                // Node is not a leaf: update the mass and centre of mass of the node
                self.nodes[insert.target_node.index].centre_of_mass = (
                    (self.nodes[insert.target_node.index].centre_of_mass.0
                        * (self.nodes[insert.target_node.index].mass as f64)
                        + body_x)
                        / (self.nodes[insert.target_node.index].mass + 1) as f64,
                    (self.nodes[insert.target_node.index].centre_of_mass.1
                        * (self.nodes[insert.target_node.index].mass as f64)
                        + body_y)
                        / (self.nodes[insert.target_node.index].mass + 1) as f64,
                );
                self.nodes[insert.target_node.index].mass += 1;
                // Add the particle to the appropriate child
                let child_desc: NodeDesc = self.get_child_node(body_x, body_y, &insert.target_node);
                let insert_as_child: Insert = Insert {
                    body_x: body_x,
                    body_y: body_y,
                    target_node: child_desc,
                };
                insert_stack.push(insert_as_child);
            }
            // print tree
            // log(self.print(0, 0).as_str());
        }
    }
}
