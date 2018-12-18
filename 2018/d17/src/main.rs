extern crate regex;

use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::collections::{VecDeque};
use regex::Regex;

fn line_to_span(line: String) -> Result<(bool, [usize; 3]),Box<std::error::Error>> {
    let re = Regex::new(r"\d+")?;
    let mut rv = [0; 3];
    let is_x = line.as_bytes()[0] == b'x';
    for (i, m) in re.find_iter(&line).enumerate().take(3) {
        rv[i] = m.as_str().parse()?;
    }
    Ok((is_x, rv))
}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("17.txt")?;
    let reader = BufReader::new(fd);
    let mut input = vec![];
    for line in reader.lines().filter_map(Result::ok) {
        input.push(line_to_span(line)?);
    }
    let start = std::time::Instant::now();
    let (min_x, max_x, min_y, max_y) = {
        let mut min_x = std::usize::MAX;
        let mut max_x = 0;
        let mut min_y = std::usize::MAX;
        let mut max_y = 0;
        for (is_x, ary) in input.iter() {
            if *is_x {
                min_x = min_x.min(ary[0]);
                max_x = max_x.max(ary[0]);
                min_y = min_y.min(ary[1]);
                max_y = max_y.max(ary[2]);
            } else {
                min_x = min_x.min(ary[1]);
                max_x = max_x.max(ary[2]);
                min_y = min_y.min(ary[0]);
                max_y = max_y.max(ary[0]);
            }
        }
        (min_x, max_x, min_y, max_y)
    };
    //println!("{:?}", (min_x, max_x, min_y, max_y));

    let width = (max_x - min_x) + 3;
    let height = max_y+1;
    let shift = min_x - 2;
    let mut grid = vec![vec![b'.'; width]; max_y+1];
    for (is_x, ary) in input.iter() {
        if *is_x {
            let x = ary[0] - shift;
            for y in ary[1]..ary[2]+1 {
                grid[y][x] = b'#';
            }
        } else {
            let y = ary[0];
            for x in ary[1]..ary[2]+1 {
                grid[y][x-shift] = b'#';
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((500-shift, min_y));

    while !queue.is_empty() {
        let (mut x, mut y) = queue.pop_front().unwrap();
        if grid[y][x] == b'|' && !b"#~".contains(&grid[y+1][x]) {
            continue;
        }
        // pour down
        while y < height && !b"#~".contains(&grid[y][x]) {
            grid[y][x] = b'|';
            y += 1;
        }
        y -= 1;
        if y >= height-1 {
            continue;
        }
        // pour sideways
        let sx = x;
        let left = loop {
            if grid[y][x] == b'#' {
                break Some(x+1);
            }
            grid[y][x] = b'|';
            if x <= 0 { break None; }
            if !b"#~".contains(&grid[y+1][x]) {
                queue.push_back((x, y+1));
                break None;
            }
            x -= 1;
        };
        x = sx;
        let right = loop {
            if grid[y][x] == b'#' {
                break Some(x);
            }
            grid[y][x] = b'|';
            if x >= width { break None; }
            if !b"#~".contains(&grid[y+1][x]) {
                queue.push_back((x,y+1));
                break None;
            }
            x += 1;
        };
        // if we've filled a layer
        if let (Some(left), Some(right)) = (left, right) {
            queue.push_back((sx, y-1));
            for x in left..right {
                grid[y][x] = b'~';
            }
        }
        //println!("{} {}", x,y);
    }

    let mut water = 0;
    let mut water_retain = 0;
    for row in grid[min_y..max_y+1].iter() {
        water += row.iter().filter(|c| b"|~".contains(c)).count();
        water_retain += row.iter().filter(|&&c| c == b'~').count();
    }
    println!("elapsed: {:?}", start.elapsed());
    println!("part1: {}", water);
    println!("part2: {}", water_retain);
    //for row in grid.iter() {
    //    println!("{}", std::str::from_utf8(&row).unwrap());
    //}
    Ok(())
}
