use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result2 = 0;

    let content = read_input()?;
    let mut marks = vec![0; content[0].len()];

    for bin in &content {
        for (i, ch) in bin.chars().enumerate() {
            if ch == '1' {
                marks[i] += 1;
            }
        }
    }
    for m in &marks {
        println!("{}", m);
    }
    let content_len = content.len();
    for i in 0..marks.len() {
        if marks[i] >= content_len / 2 {
            marks[i] = 1;
        } else {
            marks[i] = 0;
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for e in marks {
        if e == 1 {
            gamma = 2 * gamma + 1;
            epsilon = 2 * epsilon;
        } else {
            gamma = 2 * gamma;
            epsilon = 2 * epsilon + 1;
        }
    }
    let result1 = gamma * epsilon;

    let mut oxigen = 0;
    let mut co2 = 0;
    let mut oxigen_vec = content.clone();
    let mut co2_vec = content.clone();

    let bit_len = oxigen_vec[0].len();
    for i in 0..bit_len {
        if oxigen_vec.len() == 1 {
            break;
        }
        let ones = oxigen_vec
            .iter()
            .filter(|s| s.chars().nth(i) == Some('1'))
            .count();
        let keep = if ones * 2 >= oxigen_vec.len() {
            '1'
        } else {
            '0'
        };
        oxigen_vec = oxigen_vec
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(keep))
            .collect();
        println!("En la iteracion {} se escoge {}", i, keep);
    }
    for ch in oxigen_vec[0].chars() {
        if ch == '1' {
            oxigen = 2 * oxigen + 1;
        } else {
            oxigen = 2 * oxigen;
        }
    }
    for i in 0..bit_len {
        if co2_vec.len() == 1 {
            break;
        }
        let ones = co2_vec
            .iter()
            .filter(|s| s.chars().nth(i) == Some('1'))
            .count();
        let keep = if ones * 2 >= co2_vec.len() { '0' } else { '1' };
        co2_vec = co2_vec
            .into_iter()
            .filter(|s| s.chars().nth(i) == Some(keep))
            .collect();
    }
    for ch in co2_vec[0].chars() {
        if ch == '1' {
            co2 = 2 * co2 + 1;
        } else {
            co2 = 2 * co2;
        }
    }
    println!("{}, {}", oxigen_vec[0], oxigen);
    println!("{}, {}", co2_vec[0], co2);
    let result2 = oxigen * co2;

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Vec<String>> {
    let fd = File::open(Path::new("src/day3/input"))?;
    let reader = BufReader::new(fd);
    let mut content = Vec::new();
    for line in reader.lines() {
        let line = line?;
        content.push(line);
    }
    Ok(content)
}
