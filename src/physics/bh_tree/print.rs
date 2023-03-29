pub fn print(&self, depth: usize, node_index: usize) -> String {
    let mut output = String::new();
    if node_index == 0 && depth != 0 {
        return output;
    }
    for _ in 0..depth {
        output.push_str("  ");
    }
    let node: &Node = &self.nodes[node_index];
    output.push_str(&format!(
        "Node {}: mass {}, centre of mass ({}, {})\n",
        node_index, node.mass, node.centre_of_mass.0, node.centre_of_mass.1
    ));
    if node.top_left != 0 {
        output.push_str(&self.print(depth + 1, node.top_left));
    }
    if node.top_right != 0 {
        output.push_str(&self.print(depth + 1, node.top_right));
    }
    if node.bottom_left != 0 {
        output.push_str(&self.print(depth + 1, node.bottom_left));
    }
    if node.bottom_right != 0 {
        output.push_str(&self.print(depth + 1, node.bottom_right));
    }
    output
}
