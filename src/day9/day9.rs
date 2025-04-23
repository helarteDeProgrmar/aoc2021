use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let content = read_input()?;
    let mut low_points = Vec::new();
    let mut low_positions = Vec::new();

    for (i, row) in content.iter().enumerate() {
        for (j, &elem) in row.iter().enumerate() {
            if is_low(&content, i, j) {
                low_points.push(elem);
                low_positions.push((i, j));
            }
        }
    }

    result1 = low_points.into_iter().fold(0, |acc, x| acc + x + 1);
    println!("Result 1: {}", result1);
    let mut basin_sizes = Vec::new();
    let mut visited = HashSet::new();

    for &(i, j) in &low_positions {
        let size = explore_basin(&content, i, j, &mut visited);
        basin_sizes.push(size);
    }

    basin_sizes.sort_unstable_by(|a, b| b.cmp(a));
    result2 = basin_sizes.iter().take(3).product();
    println!("Result 2: {}", result2);
    Ok(())
}

fn is_low(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    let current = matrix[i][j];

    let mut neighbors = Vec::new();

    if i > 0 {
        neighbors.push(matrix[i - 1][j]);
    }
    if i + 1 < matrix.len() {
        neighbors.push(matrix[i + 1][j]);
    }
    if j > 0 {
        neighbors.push(matrix[i][j - 1]);
    }
    if j + 1 < matrix[i].len() {
        neighbors.push(matrix[i][j + 1]);
    }

    neighbors.into_iter().all(|n| current < n)
}
fn explore_basin(
    matrix: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut stack = vec![(i, j)];
    let mut size = 0;

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) || matrix[x][y] == 9 {
            continue;
        }

        visited.insert((x, y));
        size += 1;

        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < matrix.len() {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < matrix[x].len() {
            stack.push((x, y + 1));
        }
    }
    size
}

fn read_input() -> io::Result<Vec<Vec<i32>>> {
    let fd = File::open(Path::new("src/day9/input"))?;
    let reader = BufReader::new(fd);

    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        result.push(numbers);
    }
    Ok(result)
}

