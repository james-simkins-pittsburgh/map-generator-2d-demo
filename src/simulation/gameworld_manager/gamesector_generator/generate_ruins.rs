use rand::prelude::*;

use crate::SECTOR_SIZE;
use crate::simulation::TileType;

pub fn generate_ruins<R: Rng>(gamesector_environment_array: &mut[[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize], seeded_prng: &mut R) {

let mut row: i32;
let mut height: i32;
let mut x_start: i32;
let mut y_start: i32;

    for _n in 0..10 {

    row = seeded_prng.gen_range(0..6);
    height = seeded_prng.gen_range(0..53);

    match row {

    0..=2 => {x_start = -30 + row * 4;}

    _ => {x_start = 21 + row * 4;}
    
    }

    match height {

        0..=15 => {y_start = -30 + height;}
        16..=37 => {y_start = -30 + height + 4;}
        _ => {y_start = -30 + height + 8;}

        }
     
// This is where I left off

}

}