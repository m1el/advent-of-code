use std::time::{Instant};
use std::fs::{File};
use std::io::{Read};

fn react<I: Iterator<Item=u8>>(iter: I) -> Vec<u8> {
    let mut output = Vec::<u8>::with_capacity(50000);
    for c in iter {
        if output.last().cloned() == Some(c^0x20) {
            output.pop();
        } else if c > 0x20 {
            output.push(c);
        }
    }
    return output;
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut fd = File::open("05.txt")?;
    let mut input = Vec::<u8>::new();
    fd.read_to_end(&mut input)?;
    drop(fd);
    let start = Instant::now();
    let smaller = react(input.iter().cloned());
    let part1 = smaller.len();
    let part2 = (b'a'..=b'z').map(|unit| {
        react(smaller.iter().cloned().filter(|&c| (c|0x20) != unit)).len()
    }).min().unwrap();
    let elapsed = start.elapsed();
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("elapsed: {:?}", elapsed);
    Ok(())
}
