mod input;
mod physics;

use devtimer::DevTime;
use std::env;
use std::fs::File;

use crate::input::initialize_bodies;
use crate::physics::logger::{log_energy, log_positions};
use crate::physics::Vec2D;

// Starts a particle simulation with the given input JSON file
// and outputs the result to a binary file.
fn main() {
    let mut devtime = DevTime::new_simple();
    devtime.start();
    // Check arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!(
            "Usage: {} <simulation name>\nRequired files: <simulation name>.json",
            args[0]
        );
        return;
    }
    // sim name: "sim" unless specified by user
    let sim_name: String = match args.len() {
        1 => String::from("sim"),
        _ => args[1].clone(),
    };
    // Parse the input file into an input struct
    let constants: input::Constants =
        match input::parse_input((sim_name.clone() + ".json").as_str()) {
            Ok(input) => input,
            Err(err) => {
                println!("Error parsing input file: {}", err);
                return;
            }
        };
    // Open the position log file
    let mut position_log: File = match File::create(sim_name.clone() + ".bin") {
        Ok(file) => file,
        Err(err) => {
            println!("Error creating position log file: {}", err);
            return;
        }
    };
    // Open the energy log file
    let mut energy_log: File = match File::create(sim_name.clone() + "_energy.csv") {
        Ok(file) => file,
        Err(err) => {
            println!("Error creating energy log file: {}", err);
            return;
        }
    };
    // Initialize the positions, velocities, and accelerations
    let mut positions: Vec2D = Vec2D::new(constants.num_bodies);
    let mut velocities: Vec2D = Vec2D::new(constants.num_bodies);
    let mut accelerations: Vec2D = Vec2D::new(constants.num_bodies); // I think this data can be stored in "positions" when used
    initialize_bodies(&mut positions, &mut velocities, constants.num_bodies);
    // Initialize the Barnes-Hut tree
    let mut bh_tree: physics::bh_tree::Tree = physics::bh_tree::Tree::new(constants.num_bodies);
    // Run the simulation
    for step in 0..constants.num_steps {
        // Log the energy
        if constants.log_energy_conservation {
            log_energy(&positions, &velocities, &constants, &mut energy_log)
        }
        // Log the positions
        if step % constants.write_interval == 0 {
            log_positions(&positions, &constants, &mut position_log);
        }
        // Step forward in time
        physics::step(
            &mut positions,
            &mut velocities,
            &mut accelerations,
            &mut bh_tree,
            &constants,
        );
    }
    // Finish timing
    devtime.stop();
    // Print timing results
    println!("Time: {} us", devtime.time_in_micros().unwrap());
}
