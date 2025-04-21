use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut result1 = 0;
    let mut result2 = 0;

    let segments = read_input()?;
    let max: i32 = segments
        .iter()
        .flat_map(|s| [s.x1, s.x2, s.y1, s.y2])
        .max()
        .unwrap_or(0);
    let n: usize = (max + 1) as usize;
    let mut plane = vec![vec![0; n]; n];

    for seg in &segments {
        if seg.x1 == seg.x2 {
            let min;
            let max;
            if seg.y1 > seg.y2 {
                max = seg.y1;
                min = seg.y2
            } else {
                max = seg.y2;
                min = seg.y1;
            }
            for i in min..max + 1 {
                plane[seg.x1 as usize][i as usize] += 1;
            }
        } else if seg.y1 == seg.y2 {
            let min;
            let max;
            if seg.x1 > seg.x2 {
                max = seg.x1;
                min = seg.x2
            } else {
                max = seg.x2;
                min = seg.x1;
            }
            for i in min..max + 1 {
                plane[i as usize][seg.y1 as usize] += 1;
            }
        }
    }
    for line in plane {
        for elem in line {
            if elem >= 2 {
                result1 += 1;
            }
        }
    }

    plane = vec![vec![0; n]; n];

    for seg in &segments {
        if seg.x1 == seg.x2 {
            let min;
            let max;
            if seg.y1 > seg.y2 {
                max = seg.y1;
                min = seg.y2
            } else {
                max = seg.y2;
                min = seg.y1;
            }
            for i in min..max + 1 {
                plane[seg.x1 as usize][i as usize] += 1;
            }
        } else if seg.y1 == seg.y2 {
            let min;
            let max;
            if seg.x1 > seg.x2 {
                max = seg.x1;
                min = seg.x2
            } else {
                max = seg.x2;
                min = seg.x1;
            }
            for i in min..max + 1 {
                plane[i as usize][seg.y1 as usize] += 1;
            }
        } else if (seg.x1 - seg.x2) / (seg.y1 - seg.y2) == 1 {
            let mut diff = seg.x1 - seg.x2;
            let x_ground;
            let y_ground;
            if diff >= 0 {
                x_ground = seg.x2;
                y_ground = seg.y2;
            } else {
                diff = diff.abs();
                x_ground = seg.x1;
                y_ground = seg.y1;
            }

            for i in 0..diff + 1 {
                plane[(x_ground + i) as usize][(y_ground + i) as usize] += 1;
            }
        } else if (seg.x1 - seg.x2) / (seg.y1 - seg.y2) == -1 {
            let mut diff = seg.x1 - seg.x2;
            let x_ground;
            let y_ground;
            if diff >= 0 {
                x_ground = seg.x2;
                y_ground = seg.y2;
            } else {
                diff = diff.abs();
                x_ground = seg.x1;
                y_ground = seg.y1;
            }

            for i in 0..diff + 1 {
                plane[(x_ground + i) as usize][(y_ground - i) as usize] += 1;
            }
        }
    }
    for line in plane {
        for elem in line {
            if elem >= 2 {
                result2 += 1;
            }
        }
    }
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
    Ok(())
}

fn read_input() -> io::Result<Vec<Segment>> {
    let fd = File::open(Path::new("src/day5/input"))?;
    let reader = BufReader::new(fd);

    let mut segs = vec![];
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split("->").map(str::trim).collect();

        let coord1: Vec<&str> = parts[0].split(',').map(str::trim).collect();
        let coord2: Vec<&str> = parts[1].split(',').map(str::trim).collect();

        let x1: i32 = coord1[0].parse().unwrap_or(0);
        let y1: i32 = coord1[1].parse().unwrap_or(0);
        let x2: i32 = coord2[0].parse().unwrap_or(0);
        let y2: i32 = coord2[1].parse().unwrap_or(0);

        segs.push(Segment { x1, x2, y1, y2 })
    }
    Ok(segs)
}

struct Segment {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}
