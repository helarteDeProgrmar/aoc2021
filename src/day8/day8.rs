use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let content = read_input()?;
    let result1 = content.iter().map(amount_1_4_7_8).sum::<i32>();
    let result2 = content.iter().map(transform_to_number).sum::<i32>();

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn transform_to_number(row: &Row) -> i32 {
    let digits = search_digits(&row.numbers);
    let mut result = 0;
    for opt in &row.options {
        result = result * 10 + transform_to_digit(&digits, opt);
    }
    result
}

fn transform_to_digit(digits: &Vec<Vec<char>>, digit: &str) -> i32 {
    let mut target: Vec<char> = digit.chars().collect();
    target.sort_unstable();

    for (i, d) in digits.iter().enumerate() {
        if *d == target {
            return i as i32;
        }
    }
    0
}

fn amount_1_4_7_8(row: &Row) -> i32 {
    row.options
        .iter()
        .filter(|opt| {
            let len = opt.len();
            len == 2 || len == 3 || len == 4 || len == 7
        })
        .count() as i32
}

fn are_equivalent(s1: &str, s2: &str) -> bool {
    let mut a: Vec<char> = s1.chars().collect();
    let mut b: Vec<char> = s2.chars().collect();
    a.sort_unstable();
    b.sort_unstable();

    if a == b {
        println!(
            "{}, {}",
            a.iter().collect::<String>(),
            b.iter().collect::<String>()
        );
    }

    a == b
}

fn read_input() -> io::Result<Vec<Row>> {
    let fd = File::open(Path::new("src/day8/input"))?;
    let reader = BufReader::new(fd);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split('|').map(|s| s.trim()).collect();
        if parts.len() != 2 {
            continue;
        }

        let numbers = parts[0]
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let options = parts[1]
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        result.push(Row { numbers, options });
    }

    Ok(result)
}

struct Row {
    numbers: Vec<String>,
    options: Vec<String>,
}

fn search_digits(patterns: &[String]) -> Vec<Vec<char>> {
    let a: char;
    let b: char;
    let c: char;
    let d: char;
    let e: char;
    let f: char;
    let g: char;

    let aux8 = patterns.iter().find(|s| s.len() == 7);
    e = 0;
    c = 0;
    b = 0;
    d = 0;

    let l0 = vec![a, b, c, e, f, g];
    let l1 = vec![c, f];
    let l2 = vec![a, c, d, e, g];
    let l3 = vec![a, c, d, f, g];
    let l4 = vec![b, c, d, f];
    let l5 = vec![a, b, d, f, g];
    let l6 = vec![a, b, d, e, f, g];
    let l8 = vec![a, b, c, d, e, f, g];
    let l9 = vec![a, b, c, d, f, g];
    vec![l0, l1, l2, l3, l4, l5, l6, l7, l8, l9]
}
