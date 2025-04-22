use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut positions = read_input()?;
    positions.sort_unstable();

    let median = positions[positions.len() / 2];

    result1 = positions.iter().map(|&x| (x - median).abs()).sum();

    let min_pos = positions[0];
    let max_pos = positions[(positions.len() - 1) as usize];

    let mut min_fuel = i64::MAX;
    for target in min_pos..=max_pos {
        let fuel: i64 = positions
            .iter()
            .map(|&pos| {
                let distance = (pos - target).abs() as i64;
                distance * (distance + 1) / 2
            })
            .sum();

        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    result2 = min_fuel;
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Vec<i32>> {
    let fd = File::open(Path::new("src/day7/input"))?;
    let reader = BufReader::new(fd);
    let result = reader
        .lines()
        .next()
        .unwrap()?
        .split(",")
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect();
    Ok(result)
}
