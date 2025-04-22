use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2: i128 = 0;

    let mut fishes = read_input()?;

    for i in 0..80 {
        for j in 0..fishes.len() {
            if fishes[j] == 0 {
                fishes[j] = 6;
                fishes.push(8);
            } else {
                fishes[j] -= 1;
            }
        }
        println!("{}", i);
    }
    result1 = fishes.len();
    fishes = read_input()?;
    result2 = fishes
        .into_iter()
        .fold(0, |acc, x| acc + amount_of_fishes(x, 256));
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}
fn amount_of_fishes(initial_age: i32, total_generations: i32) -> i128 {
    let mut cache = HashMap::new();

    fn helper(age: i32, gens_left: i32, cache: &mut HashMap<(i32, i32), i128>) -> i128 {
        if gens_left <= age {
            return 1;
        }

        if let Some(&val) = cache.get(&(age, gens_left)) {
            return val;
        }

        let remaining = gens_left - age - 1;
        let new_fish = helper(6, remaining, cache) + helper(8, remaining, cache);

        cache.insert((age, gens_left), new_fish);
        new_fish
    }

    helper(initial_age, total_generations, &mut cache)
}
fn read_input() -> io::Result<Vec<i32>> {
    let fd = File::open(Path::new("src/day6/input"))?;
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
