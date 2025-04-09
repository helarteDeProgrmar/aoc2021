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

    let content_len = content.len();
    for i in 0..marks.len() {
        if marks[i] > content_len / 2 {
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
        println!("{}", line);
        content.push(line);
    }
    Ok(content)
}
