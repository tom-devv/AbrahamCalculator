
use std::ffi::c_float;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use crate::entity::{EntityProps, Location, powers, projectile};
use crate::permutations::permutations;

mod permutations;
mod entity;



pub const GRAVITY: f64 = 0.0399999991059303;
pub const DRAG: f64 = 0.9800000190734863;

fn main() {

    // Max Power
    let max_power = 2;

    let mut results:Vec<Vec<Location>> = Vec::new();
    let mut powers = powers();
    let projectile = projectile();
    let data: Vec<i32> = (0..max_power).collect();

    // Iterate through all possible combinations of TNT|Powers
    for p in permutations(&data[..], powers.len()){


        // Set each powers TNT value, according to the combination
        for (i,n) in p.iter().enumerate() {
            powers[i].amount = *n;
        }

        // Projectile with updated velocities
        let projectile = compound(&powers, projectile);

        // Positions of projectile during its lifespan
        let locations = positions(projectile, p);


        results.push(locations)

    }
    write_file(results);

}


fn compound(powers: &Vec<EntityProps>, mut projectile: EntityProps) -> EntityProps{

    // Apply the velocity from each power cumulatively
    for(_index ,power ) in powers.iter().enumerate() {


        let x = projectile.x - power.x;
        let y = projectile.y - power.y;
        let z = projectile.z - power.z;

        let hyp: c_float = ((x * x + y * y + z * z) as c_float).sqrt();
        let exposure:f64 = (-1.0 / 8.0) + (1.0 / hyp as f64);

        // Power is > 8 blocks away or at the same point
        if exposure == 0.0 || exposure >= 64.0 {
            println!("{:?} does not effect TNT", power);
        }

        else {
            // Update the projectiles velocities
            projectile.vy = (y * exposure * power.amount as f64) + projectile.vy;
            projectile.vx = (x * exposure * power.amount as f64) + projectile.vx;
            projectile.vz = (z * exposure * power.amount as f64) + projectile.vz;
        }
    }
    // After all velocities from powers have been given to the projectile, return it.
    projectile

}

fn positions(mut projectile: EntityProps, combo: Vec<i32>) -> Vec<Location> {
    let mut locations: Vec<Location> =  Vec::new();

    for i in 0..projectile.ticks {

        if projectile.y < 1.0 {
            println!("Projectile has gone below zero");
            break;
        }

        projectile.vy = projectile.vy - GRAVITY;
        projectile.y = projectile.y + projectile.vy;
        projectile.vy = projectile.vy * DRAG;

        locations.push(Location {
            x: projectile.x,
            y: projectile.y,
            z: projectile.z,
            vx: projectile.vx,
            vy: projectile.vy,
            vz: projectile.vz,
            tick: i,
            combo: combo.clone()
        });
    }
    locations
}

fn write_file(paths: Vec<Vec<Location>>) {
    File::create("results.json").expect("Failed to create results file");
    let mut file =OpenOptions::new()
        .write(true)
        .open("results.json")
        .unwrap();
    let json = serde_json::to_string(&paths).expect("Cannot convert to JSON");
    file.write(json.as_ref()).expect("Failed to write to file");


}


