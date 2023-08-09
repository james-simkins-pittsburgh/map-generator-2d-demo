// This function deterministically gets a sector seed from the gameworld seed.

pub fn generate_sector_seed_num_from_gameworld_seed_num(
    gameworld_seed_num: u64,
    x_coordinate: i32,
    y_coordinate: i32
) -> u64 {
    // First we need to convert our coordinates to u64.

    let x_coordinate_to_u64: u64 = if x_coordinate < 0 {
        u64::MAX - (x_coordinate.abs() as u64)
    } else {
        x_coordinate as u64
    };

    let y_coordinate_to_u64: u64 = if y_coordinate < 0 {
        u64::MAX - (y_coordinate.abs() as u64)
    } else {
        y_coordinate as u64
    };

    // Now we add a big number based on the coordinated to the gameworld seed number.
    // If this makes it larger than u64 that's fine since we just wrapping_add it.
    // 412 is for Pittsburgh pride! The other numbers are just large primes.

    gameworld_seed_num.wrapping_add(
        (412_u64).wrapping_add(
            (3943_u64)
                .wrapping_mul(x_coordinate_to_u64)
                .wrapping_add((4211_u64).wrapping_mul(y_coordinate_to_u64))
        )
    )
}

pub fn point_in_triangle(
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32
) -> bool {
    if
        triangle_area(x1, y1, x2, y2, x3, y3) ==
        triangle_area(x, y, x2, y2, x3, y3) +
            triangle_area(x1, y1, x, y, x3, y3) +
            triangle_area(x1, y1, x2, y2, x, y)
    {
        true
    } else {
        false
    }
}

fn triangle_area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 {
    (0.5) * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs()
}
