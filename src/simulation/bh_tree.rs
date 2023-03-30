use self::nodes::Node;

mod construction;
mod nodes;

pub struct Tree {
    pub nodes: Vec<Node>,
    num_nodes: usize,
    vec_size: usize,
    pub root_half_width: f64,
    pub root_centre: (f64, f64),
}
impl Tree {
    pub fn new() -> Tree {
        let mut nodes: Vec<Node> = Vec::with_capacity(1);
        nodes.push(Node::new());
        Tree {
            nodes,
            num_nodes: 1,
            vec_size: 0,
            root_half_width: 0.0,
            root_centre: (0.0, 0.0),
        }
    }
}
