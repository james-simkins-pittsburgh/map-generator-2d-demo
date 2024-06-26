use bevy::prelude::*;
use rand::prelude::*;

// These use declaration are just to make the code more readable.

use crate::SECTOR_SIZE;
use crate::simulation::InputSimulationUpdates;
use crate::simulation::TileType;
use crate::simulation::SectorBiome;
use crate::simulation::SectorBaseType;
use crate::simulation::GamesectorBasics;
use crate::simulation::GamesectorUnits;
use crate::simulation::UnitAttributes;
use super::InitializationType;

pub (super) mod generate_ruins;

// This function generates a new sector and populates it with a procedurally generated environmental map and the other components it needs.

pub fn generate_sector(
    mut generate_new_sector_event: EventReader<crate::GenerateNewSector>,
    mut sector_to_be_generated: ResMut<super::SectorToBeGenerated>,
    gameworld_seed: Res<crate::GameworldSeed>,
    mut commands: Commands,
    mut tile_control: ResMut<crate::graphics::tile_map::TileControlForSectorSwitch>
) {
    // If there are events recorded in generate sector then the rest of the function runs.

    if !generate_new_sector_event.is_empty() {
        // This iterates over the vector of tuples of coordinates of the gamesectors to generated.

        for new_sector_list in &sector_to_be_generated.sector_to_be_generated_list {
            // This returns into a tuple the data for a new gamesector.

            let new_sector_basics = generate_map(
                //  A gamesector is generated from its coordinates and a gameworld worldseed. Everything else is procedural.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &gameworld_seed.gameworld_seed_num
            );

            let new_sector_units = generate_units(
                // Units are generated or loaded based on the initialization type.
                // Sector coordinates are provided in case the sector already exists and units must be loaded.

                new_sector_list.0.clone(),
                new_sector_list.1.clone(),
                &new_sector_list.2
            );

            // This creates a new gamesector entity in the ECS and pushes the data from the tuple to it.
            commands.spawn(super::super::GamesectorBundle {
                gamesector_basics: GamesectorBasics {
                    // This stores the data of what tile goes where in the new sector's tile map component.
                    tile_array: new_sector_basics.0,

                    // This stores the sector's biome in the tile map component.
                    sector_biome: new_sector_basics.1,

                    // This stores the sector's base type in the gameworld.
                    sector_base_type: new_sector_basics.2,

                    // This stores the sector's coordinates in the gameworld.
                    sector_coordinates: new_sector_basics.3,
                },

                // This copies over the units for the new sector.
                gamesector_units: GamesectorUnits { unit_vec: new_sector_units },

                input_simulation_updates: InputSimulationUpdates {arriving_unit_vec: Vec::new(), new_instructions_vec: Vec::new()},
                output_to_travelator: crate::simulation::OutputToTravelator {departing_units: [(false, crate::simulation::UnitAttributes::default()); 24]},

            });

        
        }
        
        // This is test code saying the graphics can make its world now.
        tile_control.gamesector_generated = true;

        // This clears the event so sectors aren't generated more than once.
        generate_new_sector_event.clear();
        sector_to_be_generated.sector_to_be_generated_list.clear();

        
    }
}

// This function does the actual procedural generation of the gameworld.

fn generate_map(
    x_coordinate: i32,
    y_coordinate: i32,
    gameworld_seed_num: &u64
) -> (
    [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    SectorBiome,
    SectorBaseType,
    (i32, i32),
) {
    // This initially sets the array to open.

    let mut gamesector_environment_array: [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize] = [
        [TileType::Open; SECTOR_SIZE as usize];
        SECTOR_SIZE as usize
    ];

    // This code seeds the random number generator.

    let mut seeded_prng = rand_chacha::ChaCha8Rng::seed_from_u64(
        crate::utility::generate_sector_seed_num_from_gameworld_seed_num(
            gameworld_seed_num.clone(),
            x_coordinate,
            y_coordinate
        )
    );

    // This determines sector biome.

    let sector_biome: SectorBiome;

    if
        x_coordinate.rem_euclid(4) == 3 ||
        (x_coordinate.rem_euclid(2) == 0 &&
            (y_coordinate.rem_euclid(6) < 2 || y_coordinate.rem_euclid(6) == 5)) ||
        (x_coordinate.rem_euclid(4) == 1 && y_coordinate.rem_euclid(6) == 0)
    {
        sector_biome = SectorBiome::Alpine;
    } else {
        match seeded_prng.gen_range(0..3) {
            0 => {
                sector_biome = SectorBiome::Plains;
            }
            1 => {
                sector_biome = SectorBiome::Desert;
            }
            _ => {
                sector_biome = SectorBiome::Tundra;
            }
        }
    }

    // This determines sector base type.

    let mut sector_base_type = SectorBaseType::Wild;

    if
        x_coordinate.rem_euclid(2) == 0 &&
        (y_coordinate.rem_euclid(6) == 2 || y_coordinate.rem_euclid(6) == 4)
    {
        sector_base_type = SectorBaseType::Industrialist;
    }

    if x_coordinate.rem_euclid(4) == 1 && y_coordinate.rem_euclid(2) == 1 {
        sector_base_type = SectorBaseType::Guardian;
    }

    // This draws the patches of tall grass.

    generate_patches(
        50,
        15,
        TileType::Vegetated,
        &mut gamesector_environment_array,
        &mut seeded_prng
    );

    // This draws the rocky areas.

    if sector_biome == SectorBiome::Alpine {
        generate_patches(
            20,
            150,
            TileType::Elevated,
            &mut gamesector_environment_array,
            &mut seeded_prng
        );
    } else {
        generate_patches(
            10,
            100,
            TileType::Elevated,
            &mut gamesector_environment_array,
            &mut seeded_prng
        );
    }

    // This clear a path from the edges through the center vertically and horizontally.

    if !(sector_biome == SectorBiome::Alpine) {
        clear_paths(&mut gamesector_environment_array);
    }

    // This clear an area in the center and draws ruins for a Guardian sector.

    if sector_base_type == SectorBaseType::Industrialist {
        clear_center(&mut gamesector_environment_array, 20);
    } else if sector_base_type == SectorBaseType::Guardian {
        clear_center(&mut gamesector_environment_array, 32);
        generate_ruins::generate_ruins(&mut gamesector_environment_array,  &mut seeded_prng);
    }

    // This returns the generated environment array, sector_biome, sector_base_type, and coordinates.

    (gamesector_environment_array, sector_biome, sector_base_type, (x_coordinate, y_coordinate))
}

// This function draws the patches of vegetation and elevation.

fn generate_patches<R: Rng>(
    number_of_patches: i32,
    size_of_patches: i32,
    tile_type: TileType,
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    seeded_prng: &mut R
) {
    // This is used to bound the drawing area.

    let half_sector_size_minus_ten: i32 = ((SECTOR_SIZE - 1) / 2 - 10) as i32;

    // These are the coordinates being drawn at.

    let mut x;
    let mut y;

    // The random two axes to be moved more on.
    let mut a1;
    let mut a2;

    // Positive or negative direction on that axis.
    let mut d1;
    let mut d2;

    // Loops the number of patches.

    for _p in 0..number_of_patches {
        // Picks a random start point no closer than 10 from the edge.

        x = seeded_prng.gen_range(-half_sector_size_minus_ten..half_sector_size_minus_ten + 1);
        y = seeded_prng.gen_range(-half_sector_size_minus_ten..half_sector_size_minus_ten + 1);

        // Randomly picks the preferred axis. 0 is x/-x, 1 is y/-y.
        a1 = seeded_prng.gen_range(0..2);
        a2 = seeded_prng.gen_range(0..2);

        // Randomly picks the preferred direction on each preferred axis +/-.

        if seeded_prng.gen_range(0..2) == 1 {
            d1 = 1;
        } else {
            d1 = -1;
        }

        if seeded_prng.gen_range(0..2) == 1 {
            d2 = 1;
        } else {
            d2 = -1;
        }

        // Moves the drawing point one space at random.
        // Prefers movement in two directions based on a1, d1, a2, and d2.
        // Loops the size of each pass.

        for _s in 0..size_of_patches {
            match seeded_prng.gen_range(0..10) {
                0..=1 => if x < half_sector_size_minus_ten {
                    x = x + 1;
                }
                2..=3 => if x > -half_sector_size_minus_ten {
                    x = x - 1;
                }
                4..=5 => if y < half_sector_size_minus_ten {
                    y = y + 1;
                }
                6..=7 => if y > -half_sector_size_minus_ten {
                    y = y - 1;
                }
                8 => if d1 > 0 {
                    if a1 == 0 {
                        if x < half_sector_size_minus_ten {
                            x = x + d1;
                        }
                    } else {
                        if y < half_sector_size_minus_ten {
                            y = y + d1;
                        }
                    }
                } else {
                    if a1 == 0 {
                        if x > -half_sector_size_minus_ten {
                            x = x + d1;
                        }
                    } else {
                        if y > -half_sector_size_minus_ten {
                            y = y + d1;
                        }
                    }
                }
                _ => if d2 > 0 {
                    if a2 == 0 {
                        if x < half_sector_size_minus_ten {
                            x = x + d2;
                        }
                    } else {
                        if y < half_sector_size_minus_ten {
                            y = y + d2;
                        }
                    }
                } else {
                    if a2 == 0 {
                        if x > -half_sector_size_minus_ten {
                            x = x + d2;
                        }
                    } else {
                        if y > -half_sector_size_minus_ten {
                            y = y + d2;
                        }
                    }
                }
            }

            // Writes the tile type of the patch to the new spot.

            gamesector_environment_array[(x + ((SECTOR_SIZE as i32) - 1) / 2) as usize][
                (y + ((SECTOR_SIZE as i32) - 1) / 2) as usize
            ] = tile_type;
        }
    }

    // This code goes over the generated patches and expands them outward by one.

    let mut adjacent_count;

    let mut tile_change_tracker = [[false; SECTOR_SIZE as usize]; SECTOR_SIZE as usize];

    for x_counter in 10..(SECTOR_SIZE - 10) as usize {
        for y_counter in 10..(SECTOR_SIZE - 10) as usize {
            if !(gamesector_environment_array[x_counter][y_counter] == tile_type) {
                adjacent_count = 0;

                if gamesector_environment_array[x_counter + 1][y_counter] == tile_type {
                    adjacent_count += 1;
                }
                if gamesector_environment_array[x_counter - 1][y_counter] == tile_type {
                    adjacent_count += 1;
                }
                if gamesector_environment_array[x_counter][y_counter + 1] == tile_type {
                    adjacent_count += 1;
                }

                if gamesector_environment_array[x_counter][y_counter - 1] == tile_type {
                    adjacent_count += 1;
                }

                if adjacent_count > 0 {
                    tile_change_tracker[x_counter][y_counter] = true;
                }
            }
        }
    }

    for x_counter in 10..(SECTOR_SIZE - 10) as usize {
        for y_counter in 10..(SECTOR_SIZE - 10) as usize {
            if tile_change_tracker[x_counter][y_counter] == true {
                gamesector_environment_array[x_counter][y_counter] = tile_type;
            }
        }
    }

    // This code goes over the generated patches three times and adds to any patch that has the tile type on 3 sides.

    for _pass in 1..4 {
        for x_counter in 10..(SECTOR_SIZE - 10) as usize {
            for y_counter in 10..(SECTOR_SIZE - 10) as usize {
                if !(gamesector_environment_array[x_counter][y_counter] == tile_type) {
                    adjacent_count = 0;

                    if gamesector_environment_array[x_counter + 1][y_counter] == tile_type {
                        adjacent_count += 1;
                    }
                    if gamesector_environment_array[x_counter - 1][y_counter] == tile_type {
                        adjacent_count += 1;
                    }
                    if gamesector_environment_array[x_counter][y_counter + 1] == tile_type {
                        adjacent_count += 1;
                    }

                    if gamesector_environment_array[x_counter][y_counter - 1] == tile_type {
                        adjacent_count += 1;
                    }

                    if adjacent_count > 2 {
                        gamesector_environment_array[x_counter][y_counter] = tile_type;
                    }
                }
            }
        }
    }
}

// Clears a path from edge to center. Used for all biomes except mountain.

fn clear_paths(
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize]
) {
    for y in -5..6 {
        for x in -(((SECTOR_SIZE as i32) - 1) / 2 - 9) + 10..((SECTOR_SIZE as i32) - 1) / 2 - 9 {
            gamesector_environment_array[(x + ((SECTOR_SIZE as i32) - 1) / 2) as usize][
                (y + ((SECTOR_SIZE as i32) - 1) / 2) as usize
            ] = TileType::Open;
        }
    }

    for x in -5..6 {
        for y in -(((SECTOR_SIZE as i32) - 1) / 2 - 9) + 10..((SECTOR_SIZE as i32) - 1) / 2 - 9 {
            gamesector_environment_array[(x + ((SECTOR_SIZE as i32) - 1) / 2) as usize][
                (y + ((SECTOR_SIZE as i32) - 1) / 2) as usize
            ] = TileType::Open;
        }
    }
}

// Clear a center from 0 to clear size in the the positive and negative x and y directions. Used for base sectors.

fn clear_center(
    gamesector_environment_array: &mut [[TileType; SECTOR_SIZE as usize]; SECTOR_SIZE as usize],
    clear_size: i32
) {
    for y in -clear_size..clear_size + 1 {
        for x in -clear_size..clear_size + 1 {
            gamesector_environment_array[(x + ((SECTOR_SIZE as i32) - 1) / 2) as usize][
                (y + ((SECTOR_SIZE as i32) - 1) / 2) as usize
            ] = TileType::Open;
        }
    }
}

fn generate_units(
    _x_coordinate: i32,
    _y_coordinate: i32,
    _initialization_type: &InitializationType
) -> Vec<UnitAttributes> {
    let unit_list: Vec<UnitAttributes> = Vec::new();

    unit_list
}
