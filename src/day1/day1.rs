use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let nums: Vec<i32> = read_input()?;
    let mut result = 0;
    let mut result2 = 0;

    for pair in nums.windows(2) {
        if pair[0] < pair[1] {
            result += 1;
        }
    }

    let sums: Vec<i32> = nums.windows(3).map(|w| w[0] + w[1] + w[2]).collect();

    for pair in sums.windows(2) {
        if pair[0] < pair[1] {
            result2 += 1;
        }
    }

    println!("Result 1: {}", result);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Vec<i32>> {
    let fd = File::open(Path::new("src/day1/input"))?;
    let lector = BufReader::new(fd);

    lector
        .lines()
        .map(|line| {
            let line = line?;
            let numero: i32 = line.trim().parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Error al parsear número: {}", e),
                )
            })?;
            Ok(numero)
        })
        .collect()
}
