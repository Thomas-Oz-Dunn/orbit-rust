/*
Orbital systems modelling in Rust

*/
mod donnager;
use donnager::{constants, ballistics, astro};

fn main() {
    let mass_0: f64 = constants::EARTH_MASS;
    let radius_0: f64 = constants::EARTH_RADIUS_EQUATOR; 

    let mass_1: f64 = 1.0; // kg
    let altitude: f64 = 3.0e6; // 300km, LEO
    let radius_f: f64 = radius_0 + altitude; // m
    let engine_isp: f64 = 300.0; // s

    let grav_param: f64 = mass_0 * constants::GRAV_CONST;
    let delta_v: f64 = astro::calc_orbital_velocity(grav_param, radius_f);
    let grav_acc: f64 = grav_param / radius_f.powi(2);
    let mass_ratio: f64 = ballistics::calc_mass_ratio(delta_v, engine_isp, grav_acc);
    let mass_fuel: f64 = mass_1 * mass_ratio;    
    println!("{} kg of fuel", mass_fuel);
}

/*
TODO-TD: 
N-stage trade study plots
TLE ingest
Trans Lunar Injections
Multithreading, Cloud Compute?
RK45 propogator
J2 perturbation
Launch cost calculator
Comparitive propulsion techniques
    Liquid
        Monoprop
        Biprop
    Solid
    Electric
    Nuclear Thermal
Solar System Mineralogical data base query
Interplanetary Mission Plan (optimal launch windows, porkchop plot)
$ / kg (mineral X)
*/