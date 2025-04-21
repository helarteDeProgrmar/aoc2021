use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut fishes = read_input()?;

    for i in 0..80 {
        for elem in fishes {
            if elem == 0 {
                fishes[j] = 6;
                fishes.push(8);
            } else {
                fishes[elem] -= 1;
            }
        }
    }
    result1 = fold(|acc, x| acc + x, 0, fishes);
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Vec<i32>> {
    let fd = File::open(Path::new("src/day6/input"))?;
    let reader = BufReader::new(fd);
    let mut result = reader
        .lines()
        .next()
        .split(",")
        .map(str::trim)
        .map(parse::<i32>)
        .collect();
    Ok(result)
}
