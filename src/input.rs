

use std::fs;
use rand::Rng;
use super::physics;

use serde::Deserialize;

const RANDOM_GENERATOR_BOX_SIZE: f64 = 100.0;
const RANDOM_GENERATOR_START_TEMP: f64 = 100.0; // Higher is colder

// Input structure to store JSON input file data
#[derive(Debug, Deserialize)]
pub struct Constants {
	pub num_bodies: usize,
	pub num_steps: usize,
	pub write_interval: usize,
	pub delta_t: f64,
	pub softening: f64,
	pub gravity: f64,
	pub log_energy_conservation: bool,
	pub theta: f64,
}

// Opens the input JSON file and parses the data into an Input struct
// The input JSON file must be in the following format:
// {
// 	"num_particles": 1000,
// 	"num_steps": 1000,
// 	"write_interval": 100,
// 	"delta_t": 0.01,
// 	"softening": 0.1,
// 	"gravity": 1.0,
// 	"log_energy_conservation": false,
// 	"theta": 0.5,
// }
pub fn parse_input(filename: &str) -> Result<Constants, String> {
	// Convert file to String
	let json: String = match fs::read_to_string(filename) {
		Ok(json) => json,
		Err(err) => return Err(format!("Error parsing JSON file '{}': {}", filename, err)),
	};
	// Convert the String to a string slice
	let json: &str = &json;
	// Parse the JSON string into a Constants struct
	let constant: Constants = match serde_json::from_str(json) {
		Ok(input) => input,
		Err(err) => return Err(format!("Error parsing JSON: {}", err)),
	};
	// Return the Constants struct
	Ok(constant)
}

// Adds random points to the given mutable reference to a vector of points
pub fn initialize_bodies(positions: &mut Vec<physics::Point>, velocities: &mut Vec<physics::Point>, num_bodies: usize) {
	// Allocate space for the vectors
	positions.reserve(num_bodies as usize);
	velocities.reserve(num_bodies as usize);
	// Generate random positions in a uniform distribution from -RANDOM_GENERATOR_BOX_SIZE/2 to RANDOM_GENERATOR_BOX_SIZE/2
	let mut rng = rand::thread_rng();
	for _ in 0..num_bodies {
		positions.push(physics::Point {
			x: rng.gen::<f64>() * RANDOM_GENERATOR_BOX_SIZE - RANDOM_GENERATOR_BOX_SIZE / 2.0,
			y: rng.gen::<f64>() * RANDOM_GENERATOR_BOX_SIZE - RANDOM_GENERATOR_BOX_SIZE / 2.0,
		});
	}
	// Generate random velocities in a uniform distribution from -RANDOM_GENERATOR_BOX_SIZE/RANDOM_GENERATOR_START_TEMP to RANDOM_GENERATOR_BOX_SIZE/RANDOM_GENERATOR_START_TEMP
	for _ in 0..num_bodies {
		velocities.push(physics::Point {
			x: rng.gen::<f64>() * RANDOM_GENERATOR_BOX_SIZE / RANDOM_GENERATOR_START_TEMP - RANDOM_GENERATOR_BOX_SIZE / (2.0 * RANDOM_GENERATOR_START_TEMP),
			y: rng.gen::<f64>() * RANDOM_GENERATOR_BOX_SIZE / RANDOM_GENERATOR_START_TEMP - RANDOM_GENERATOR_BOX_SIZE / (2.0 * RANDOM_GENERATOR_START_TEMP),
		});
	}
}