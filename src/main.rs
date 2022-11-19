mod physics;
mod input;

use std::fs::File;
use std::env;

// Starts a particle simulation with the given input JSON file
// and outputs the result to a binary file.
fn main() {
	// Check arguments
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		println!("Usage: {} <simulation name>\nRequired files: <simulation name>.json", args[0]);
		return;
	}
	// Parse the input file into an input struct
	let constants: input::Constants = match input::parse_input((args[1].clone() + ".json").as_str()) {
		Ok(input) => input,
		Err(err) => {
			println!("Error parsing input file: {}", err);
			return;
		}
	};
	// Open the position log file
	let mut position_log: File = match File::create(args[1].clone() + ".bin") {
		Ok(file) => file,
		Err(err) => {
			println!("Error creating position log file: {}", err);
			return;
		}
	};
	// Open the energy log file
	let mut energy_log: File = match File::create(args[1].clone() + "_energy.csv") {
		Ok(file) => file,
		Err(err) => {
			println!("Error creating energy log file: {}", err);
			return;
		}
	};
	// Initialize the positions, velocities, and accelerations
	let mut positions: Vec<physics::Point> = Vec::new();
	let mut velocities: Vec<physics::Point> = Vec::new();
	input::initialize_bodies(&mut positions, &mut velocities, constants.num_bodies);
	// Initialize the Barnes-Hut tree
	let mut bh_tree: physics::bh_tree::Tree = physics::bh_tree::Tree::new();
	// Run the simulation
	for step in 0..constants.num_steps {
		// Log the energy
		if constants.log_energy_conservation {
			physics::logger::log_energy(&positions, &velocities, &constants, &mut energy_log)
		}
		// Log the positions
		if step % constants.write_interval == 0 {
			physics::logger::log_positions(&positions, &mut position_log);
		}
		// Step forward in time
		physics::step(&mut positions, &mut velocities, &mut bh_tree, &constants);
	}
}