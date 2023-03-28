pub fn add_node_acceleration(
    body: (&f64, &f64),
    bh_tree: &Tree,
    node_index: usize,
    s: f64,
) -> (f64, f64) {
    let mut acceleration: (f64, f64) = (0.0, 0.0);
    let (body_x, body_y): (&f64, &f64) = body;
    // Calculate the distance between the particle and the node
    let dx: f64 = bh_tree.nodes[node_index].centre_of_mass.0 - body_x;
    let dy: f64 = bh_tree.nodes[node_index].centre_of_mass.1 - body_y;
    let d: f64 = (dx * dx + dy * dy).sqrt();
    // If the node is a leaf, add the acceleration
    if bh_tree.nodes[node_index].mass == 1 {
        // Calculate and add the acceleration (mass is 1)
        acceleration.0 += dx / (d * d * d + SOFTENING);
        acceleration.1 += dy / (d * d * d + SOFTENING);
    }
    // If the node is not a leaf, check if the node is far enough to take its centre of mass
    else {
        // Check the s/d ratio for the node
        if s / d < THETA {
            // Calculate and add the acceleration (mass is >1)
            acceleration.0 += bh_tree.nodes[node_index].mass as f64 * dx / (d * d * d + SOFTENING);
            acceleration.1 += bh_tree.nodes[node_index].mass as f64 * dy / (d * d * d + SOFTENING);
        } else {
            // Recursively calculate the acceleration
            let new_s = s / 2.0;
            if bh_tree.nodes[node_index].bottom_left != 0 {
                let added_acceleration: (f64, f64) = add_node_acceleration(
                    body,
                    bh_tree,
                    bh_tree.nodes[node_index].bottom_left,
                    new_s,
                );
                acceleration.0 += added_acceleration.0;
                acceleration.1 += added_acceleration.1;
            }
            if bh_tree.nodes[node_index].bottom_right != 0 {
                let added_acceleration: (f64, f64) = add_node_acceleration(
                    body,
                    bh_tree,
                    bh_tree.nodes[node_index].bottom_right,
                    new_s,
                );
                acceleration.0 += added_acceleration.0;
                acceleration.1 += added_acceleration.1;
            }
            if bh_tree.nodes[node_index].top_left != 0 {
                let added_acceleration: (f64, f64) =
                    add_node_acceleration(body, bh_tree, bh_tree.nodes[node_index].top_left, new_s);
                acceleration.0 += added_acceleration.0;
                acceleration.1 += added_acceleration.1;
            }
            if bh_tree.nodes[node_index].top_right != 0 {
                let added_acceleration: (f64, f64) = add_node_acceleration(
                    body,
                    bh_tree,
                    bh_tree.nodes[node_index].top_right,
                    new_s,
                );
                acceleration.0 += added_acceleration.0;
                acceleration.1 += added_acceleration.1;
            }
        }

        // print the acceleration
        // println!("acc: {} {}", acceleration.0, acceleration.1);
    }
    acceleration
}
