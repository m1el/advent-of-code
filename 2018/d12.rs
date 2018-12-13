use std::collections::{HashMap,VecDeque};
use std::io::{BufReader, BufRead};
use std::fs::{File};

fn parse(s: &[u8]) -> usize {
    let mut n = 0;
    for c in s.iter().cloned() {
        n = n * 2 + if c == b'#' { 1 } else { 0 };
    }
    n
}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("12.txt")?;
    let mut reader = BufReader::new(fd);
    let mut state = VecDeque::new();
    let mut line = String::new();
    &reader.read_line(&mut line);
    state.extend(line.trim()[15..].as_bytes().iter());
    reader.read_line(&mut line)?;
    let mut reps = HashMap::new();
    for line in reader.lines().filter_map(Result::ok) {
        let line = line.as_bytes();
        let start = parse(&line[0..5]);
        let rep = line[9];
        reps.insert(start, rep);
    }
    let mut off = 4_isize;
    for _ in 0..4 {
        state.push_back(b'.');
        state.push_front(b'.');
    }
    for step in 0..100 {
        let mut next = VecDeque::new();
        next.push_back(b'.');
        next.push_back(b'.');
        for _ in 0..4 {
            state.push_back(b'.');
        }
        for idx in 0..state.len()-5 {
            let mut chunk = [0; 5];
            for i in 0..5 {
                chunk[i] = state[i+idx];
            }
            let rep = *reps.get(&parse(&chunk)).unwrap_or(&b'.');
            //println!("{} {}", std::str::from_utf8(&chunk)?, rep as char);
            next.push_back(rep);
        }
        //panic!();
        state = next;
        let copy: Vec<u8> = state.clone().into();
        println!("{} {}", off, std::str::from_utf8(&copy)?);
    }
    let part1 = state.iter().enumerate()
             .filter(|(_idx, &val)| val == b'#')
             .map(|(idx, _val)| (idx as isize) - off)
             .sum::<isize>();

    let p2 = state.iter().filter(|&&v| v == b'#').count() as isize;
    println!("{}", part1);
    println!("{}", p2*(50_000_000_000-100)+part1);
    Ok(())
}
