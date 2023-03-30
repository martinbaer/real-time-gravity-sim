pub struct Node {
    pub centre_of_mass: (f64, f64),
    pub mass: usize,
    pub top_left: usize,
    pub top_right: usize,
    pub bottom_left: usize,
    pub bottom_right: usize,
}
impl Node {
    pub fn new() -> Node {
        Node {
            centre_of_mass: (0.0, 0.0),
            mass: 0,
            top_left: 0,
            top_right: 0,
            bottom_left: 0,
            bottom_right: 0,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.top_left == 0 && self.top_right == 0 && self.bottom_left == 0 && self.bottom_right == 0
    }
}
pub struct NodeDesc {
    pub index: usize,
    pub centre_x: f64,
    pub centre_y: f64,
    pub half_width: f64,
}
impl NodeDesc {
    pub fn new(half_width: f64) -> NodeDesc {
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
