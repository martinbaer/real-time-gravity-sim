use std::fs::File;
use std::io::Write;

use crate::physics::Point;
use crate::input::Constants;


// Logs the energy conservation given positions, velocities, and a file to write to.
//
// The energy conservation is defined as the sum of the kinetic energy and the potential energy.
// The kinetic energy is defined as 1/2 * v^2
// The potential energy is defined as -G / r
// The total energy is defined as the sum of the kinetic energy and the potential energy
pub fn log_energy(positions: &Vec<Point>, velocities: &Vec<Point>, constants: &Constants, file: &mut File) {
	// Calculate the total energy
	let mut total_kinetic_energy: f64 = 0.0;
	let mut total_potential_energy: f64 = 0.0;
	for i in 0..positions.len() {
		// Calculate the kinetic energy
		let kinetic_energy: f64 = 0.5 * (velocities[i].x + velocities[i].y);
		// Calculate the potential energy
		let mut potential_energy: f64 = 0.0;
		for j in 0..positions.len() {
			if i != j {
				let r: f64 = ((positions[i].x - positions[j].x).powi(2) + (positions[i].y - positions[j].y).powi(2)).sqrt();
				potential_energy += -constants.gravity / r;
			}
		}
		// Add the kinetic and potential energy to the total
		total_kinetic_energy += kinetic_energy;
		total_potential_energy += potential_energy;
	}
	// Calculate the total energy
	let total_energy: f64 = total_kinetic_energy + total_potential_energy;
	// Save the total energy to the text file
	match file.write(format!("{}\n", total_energy).as_bytes()) {
		Ok(_) => (),
		Err(err) => println!("Error writing to energy log file: {}", err),
	};
}

// Logs the positions of the bodies to a binary file
pub fn log_positions(positions: &Vec<Point>, file: &mut File) {
	// Write the positions
	for i in 0..positions.len() {
		match file.write(&positions[i].x.to_le_bytes()) {
			Ok(_) => (),
			Err(err) => {
				println!("Error writing to position log file: {}", err);
				return;
			}
		};
		match file.write(&positions[i].y.to_le_bytes()) {
			Ok(_) => (),
			Err(err) => {
				println!("Error writing to position log file: {}", err);
				return;
			}
		};
	}
}