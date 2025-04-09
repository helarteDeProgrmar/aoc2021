use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let result1 = read_input()?;
    let result2 = read_input2()?;
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<i32> {
    let file = File::open(Path::new("src/day2/input"))?;
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap_or("");
        let amount_str = parts.next().unwrap_or("0");
        let amount: i32 = amount_str.parse().unwrap_or(0);
        println!("{}, {}", amount, instruction);
        match instruction {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => {}
        }
    }

    Ok(horizontal * depth)
}

fn read_input2() -> io::Result<i32> {
    let file = File::open(Path::new("src/day2/input"))?;
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap_or("");
        let amount_str = parts.next().unwrap_or("0");
        let amount: i32 = amount_str.parse().unwrap_or(0);
        println!("{}, {}", amount, instruction);
        match instruction {
            "forward" => {horizontal += amount; depth += aim*amount},
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => {}
        }
    }

    Ok(horizontal * depth)
}
