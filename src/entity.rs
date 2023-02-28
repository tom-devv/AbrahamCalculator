#[derive(Debug, Clone, Copy)]
pub struct EntityProps {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub amount: i32,
    pub ticks: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub tick: i32,
    pub combo: Vec<i32>
}


pub fn powers() -> Vec<EntityProps>{
    let mut powers: Vec<EntityProps> = Vec::new();
    // Create powers however you want.

    // Iteration using regular intervals
    let z = 6.0;
    let range = 5;
    for i in 0..range {
        let power = EntityProps {
            x: 0.0,
            y: 254.23,
            z: z - i as f64, // Z Coordinate will move backwards 5 times.
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            amount: 0, //  Not important
            ticks: 0,  //  Not important
        };
        powers.push(power)
    }

    // Create each power separate and push to array;
    powers
}

pub fn projectile() -> EntityProps {
    let projectile = EntityProps {
        x: 0.0,
        y: 254.50,
        z: 0.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
        amount: 0,
        ticks: 10,
    };
    projectile
}