use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut bingo = read_input()?;
    let way = bingo.way.clone();

    for num in way {
        // Marca el número en cada matriz
        bingo.matrix_set = bingo
            .matrix_set
            .into_iter()
            .map(|matrix| {
                matrix
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|e| if e == num { -e } else { e })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        for m in &bingo.matrix_set {
            // check si hay fila ganadora
            if m.iter().any(|row| row.iter().all(|e| *e < 0)) {
                result1 = num
                    * m.iter().fold(0, |acc, row| {
                        acc + row.iter().filter(|e| **e >= 0).sum::<i32>()
                    });
                break;
            }
        }

        if result1 != 0 {
            break; // salimos una vez tengamos el primer resultado
        }
    }
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Bingo> {
    let fd = File::open(Path::new("src/day1/input"))?;
    let reader = BufReader::new(fd);
    let mut lines = reader.lines();

    let way_line = lines.next().unwrap()?;
    let way: Vec<i32> = way_line
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut matrix_set = Vec::new();
    let mut current_matrix = Vec::new();

    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            if !current_matrix.is_empty() {
                matrix_set.push(current_matrix);
                current_matrix = Vec::new();
            }
        } else {
            let row: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            current_matrix.push(row);
        }
    }

    if !current_matrix.is_empty() {
        matrix_set.push(current_matrix);
    }

    Ok(Bingo { way, matrix_set })
}
struct Bingo {
    way: Vec<i32>,
    matrix_set: Vec<Vec<Vec<i32>>>,
}
