use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::collections::{HashMap};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

type Grid = Vec<Vec<u8>>;

fn next(ch: u8, trees: usize, lumbers: usize) -> u8 {
    match ch {
        b'.' => if trees > 2 { b'|' } else { b'.' }
        b'|' => if lumbers > 2 { b'#' } else { b'|' }
        b'#' => if trees > 0 && lumbers > 0 { b'#' } else { b'.' }
        _ => ch
    }
}

fn count_neighbors(grid: &Grid, sx: usize, sy: usize) -> (usize, usize) {
    let h = grid.len();
    let w = grid[0].len();
    let range_y = sy.saturating_sub(1)..(sy+2).min(h);
    let range_x = sx.saturating_sub(1)..(sx+2).min(w);
    let mut trees = 0;
    let mut lumbers = 0;
    for y in range_y {
        for x in range_x.clone() {
            if (x, y) == (sx, sy) { continue }
            match grid[y][x] {
                b'|' => trees += 1,
                b'#' => lumbers += 1,
                _ => {},
            }
        }
    }
    (trees, lumbers)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn score(grid: &Grid) -> usize {
    let mut trees = 0;
    let mut lumbers = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match grid[y][x] {
                b'|' => trees += 1,
                b'#' => lumbers += 1,
                _ => {},
            }
        }
    }
    trees * lumbers
}

fn n_steps<'a>(grid: &'a mut Grid, scratch: &'a mut Grid, finish: usize) {
    let mut seen = HashMap::<u64,usize>::new();
    let mut end = None;
    for it in 0..finish {
        if let Some(end) = end {
            if end == it { break }
        } else {
            let hash = calculate_hash(&grid);
            if let Some(prev) = seen.insert(hash, it) {
                let left = (finish - it) % (it - prev);
                end = Some(it + left);
            }
        }

        for y in 0..grid.len() {
            let row = &mut scratch[y];
            for x in 0..grid[y].len() {
                let (trees, lumbers) = count_neighbors(&grid, x, y);
                row[x] = next(grid[y][x], trees, lumbers);
            }
        }
        std::mem::swap(grid, scratch);
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("18.txt")?;
    let reader = BufReader::new(fd);
    let mut grid = vec![];
    for line in reader.lines().filter_map(Result::ok) {
        let mut line = line.trim().to_string().into_bytes();
        grid.push(line);
    }

    let start = std::time::Instant::now();
    let mut scratch = grid.clone();

    let mut grid1 = grid.clone();
    n_steps(&mut grid1, &mut scratch, 10);
    let part1 = score(&grid1);

    n_steps(&mut grid, &mut scratch, 1000_000_000);
    let part2 = score(&grid);

    println!("elapsed: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
