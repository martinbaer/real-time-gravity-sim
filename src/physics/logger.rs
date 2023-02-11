use std::fs::File;
use std::io::Write;

use crate::physics::Point;
use crate::input::Constants;

use super::Vec2D;


// Logs the energy conservation given positions, velocities, and a file to write to.
//
// The energy conservation is defined as the sum of the kinetic energy and the potential energy.
// The kinetic energy is defined as 1/2 * v^2
// The potential energy is defined as -G / r
// The total energy is defined as the sum of the kinetic energy and the potential energy
pub fn log_energy(positions: &Vec2D, velocities: &Vec2D, constants: &Constants, file: &mut File) {
	// Calculate the total energy
	let mut total_kinetic_energy: f64 = 0.0;
	let mut total_potential_energy: f64 = 0.0;
	for i in 0..constants.num_bodies {
		// Calculate the kinetic energy
		let kinetic_energy: f64 = 0.5 * (velocities.x[i].powi(2) + velocities.y[i].powi(2));
		// Calculate the potential energy
		let mut potential_energy: f64 = 0.0;
		for j in 0..constants.num_bodies {
			if i != j {
				let r: f64 = ((positions.x[i] - positions.x[j]).powi(2) + (positions.y[i] - positions.y[j]).powi(2)).sqrt();
				potential_energy -= constants.gravity / r;
			}
		}
		// Add the kinetic and potential energy to the total
		total_kinetic_energy += kinetic_energy;
		total_potential_energy += potential_energy;
	}
	// Calculate the total energy
	let total_energy: f64 = total_kinetic_energy + total_potential_energy;
	// Save the total energy, kinetic energy and potential energy to the text file
	match file.write(format!("{},{},{}\n", total_energy, total_kinetic_energy, total_potential_energy).as_bytes()) {
		Ok(_) => (),
		Err(err) => panic!("Error writing to energy log file: {}", err),
	}
}

// Logs the positions of the bodies to a binary file
pub fn log_positions(positions: &Vec2D, constants: &Constants, file: &mut File) {
	// Write the positions
	for i in 0..constants.num_bodies {
		match file.write(&positions.x[i].to_le_bytes()) {
			Ok(_) => (),
			Err(err) => panic!("Error writing to position log file: {}", err),
		}
	}
	for i in 0..constants.num_bodies {
		match file.write(&positions.y[i].to_le_bytes()) {
			Ok(_) => (),
			Err(err) => panic!("Error writing to position log file: {}", err),
		}
	}
}