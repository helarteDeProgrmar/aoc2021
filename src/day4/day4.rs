use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut bingo = read_input()?;

    for num in bingo.way {
        println!("The current number is{}", num);
        for matrix in &mut bingo.matrix_set {
            for row in matrix {
                for val in row {
                    if *val == num {
                        (*val) = -(*val);
                        println!("{}", *val);
                    }
                }
            }
        }

        for matrix in &bingo.matrix_set {
            for row in matrix {
                if row.iter().all(|val| *val < 0) {
                    result1 = num * sum_all(matrix);
                    break;
                }
            }
            let num_columns = matrix[0].len();
            for i in 0..num_columns {
                if matrix.iter().all(|row| row[i] < 0) {
                    result1 = num * sum_all(matrix);
                    break;
                }
            }
        }

        if result1 != 0 {
            break;
        }
    }

    bingo = read_input()?;
    for num in bingo.way {
        println!("The current number is{}", num);
        for matrix in &mut bingo.matrix_set {
            for row in matrix {
                for val in row {
                    if *val == num {
                        (*val) = -(*val);
                        println!("{}", *val);
                    }
                }
            }
        }
        if bingo.matrix_set.len() == 1 {
            result2 = num * sum_all(&bingo.matrix_set[0]);
            break;
        }
        //for (idx_matrix, matrix) in (&mut bingo.matrix_set).into_iter().enumerate() {
        //    for row in matrix {
        //        if row.iter().all(|val| *val < 0) {
        //            bingo.matrix_set.remove(idx_matrix);
        //            break;
        //        }
        //    }
        //    let num_columns = matrix[0].len();
        //    for i in 0..num_columns {
        //        if matrix.iter().all(|row| row[i] < 0) {
        //            bingo.matrix_set.remove(idx_matrix);
        //            break;
        //        }
        //    }
        //}
        //bingo.matrix_set =
        bingo.matrix_set.retain(|matrix| {
            let row_has_all_neg = matrix.iter().any(|row| row.iter().all(|val| *val < 0));
            if row_has_all_neg {
                return false;
            }

            let num_columns = matrix[0].len();
            for i in 0..num_columns {
                if matrix.iter().all(|row| row[i] < 0) {
                    return false;
                }
            }

            true
        });
    }
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Bingo> {
    let fd = File::open(Path::new("src/day4/input"))?;
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
fn sum_all(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in matrix {
        for val in row {
            if *val >= 0 {
                sum += *val;
            }
        }
    }
    sum
}
struct Bingo {
    way: Vec<i32>,
    matrix_set: Vec<Vec<Vec<i32>>>,
}
