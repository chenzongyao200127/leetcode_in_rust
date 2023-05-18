use std::cmp::Ordering;

type Point = (f64, f64);

fn minimum_stations(ships: &mut Vec<Point>, d: f64) -> usize {
    ships.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));

    let mut stations = 0;
    let mut right_boundary = f64::NEG_INFINITY;

    for ship in ships.iter() {
        if ship.0 <= right_boundary {
            continue;
        }

        let ship_right_boundary = ship.0 + (d * d - ship.1 * ship.1).sqrt();

        stations += 1;
        right_boundary = ship_right_boundary;
    }

    stations
}

fn main() {
    let mut ships = vec![(1.0, 1.0), (3.0, 1.0), (7.0, 1.0), (10.0, 1.0), (14.0, 1.0)];
    let d = 3.0;
    let min_stations = minimum_stations(&mut ships, d);
    println!("the minimum stations is : {}", min_stations);
}