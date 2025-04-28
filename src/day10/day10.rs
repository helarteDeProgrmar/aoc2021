use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let content = read_input()?;

    let mut result2s: Vec<i128> = Vec::new();
    for line in content {
        let (score, control) = analize_line(&line);
        if control {
            result2s.push(score);
        } else {
            result1 += score;
        }
    }
    result2s.sort();
    result2 = result2s[(result2s.len() / 2) as usize];
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn analize_line(line: &str) -> (i128, bool) {
    let mut stack = Vec::new();
    for character in line.chars() {
        match character {
            '(' => stack.push(character),
            '{' => stack.push(character),
            '<' => stack.push(character),
            '[' => stack.push(character),
            ')' => {
                if '(' != stack.pop().unwrap_or(' ') {
                    println!("{}, {}", ")", character);
                    return (3, false);
                }
            }
            '}' => {
                if '{' != stack.pop().unwrap_or(' ') {
                    println!("{}, {}", "}", character);
                    return (1197, false);
                }
            }
            '>' => {
                if '<' != stack.pop().unwrap_or(' ') {
                    println!("{}, {}", ">", character);
                    return (25137, false);
                }
            }
            ']' => {
                if '[' != stack.pop().unwrap_or(' ') {
                    println!("{}, {}", "]", character);
                    return (57, false);
                }
            }
            _ => todo!(),
        }
    }
    let mut score: i128 = 0;
    while let Some(open) = stack.pop() {
        let val = match open {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
        score = score * 5 + val;
    }
    println!("{}", score);
    (score, true)
}

fn read_input() -> io::Result<Vec<String>> {
    let fd = File::open(Path::new("src/day10/input"))?;
    let reader = BufReader::new(fd);

    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        result.push(line.clone());
    }
    Ok(result)
}
