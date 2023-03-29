use crate::{constants::GRAVITY, log};

// Logs the energy conservation given positions, velocities, and a file to write to.
//
// The energy conservation is defined as the sum of the kinetic energy and the potential energy.
// The kinetic energy is defined as 1/2 * v^2
// The potential energy is defined as -G / r
// The total energy is defined as the sum of the kinetic energy and the potential energy
pub fn log_energy(x: &Vec<f64>, y: &Vec<f64>, vx: &Vec<f64>, vy: &Vec<f64>, num_bodies: usize) {
    // Calculate the total energy
    let mut total_kinetic_energy: f64 = 0.0;
    let mut total_potential_energy: f64 = 0.0;
    for i in 0..num_bodies {
        // Calculate the kinetic energy
        let kinetic_energy: f64 = 0.5 * (vx[i].powi(2) + vy[i].powi(2));
        // Calculate the potential energy
        let mut potential_energy: f64 = 0.0;
        for j in 0..num_bodies {
            if i != j {
                let r: f64 = ((x[i] - x[j]).powi(2) + (y[i] - y[j]).powi(2)).sqrt();
                potential_energy -= GRAVITY / r;
            }
        }
        // Add the kinetic and potential energy to the total
        total_kinetic_energy += kinetic_energy;
        total_potential_energy += potential_energy;
    }
    // Calculate the total energy
    let total_energy: f64 = total_kinetic_energy + total_potential_energy;
    // Save the total energy, kinetic energy and potential energy to the text file
    log(format!(
        "{},{},{}\n",
        total_energy, total_kinetic_energy, total_potential_energy
    )
    .as_str());
}
