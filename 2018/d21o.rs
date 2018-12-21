use std::collections::{HashSet};
fn main() {
    let start = std::time::Instant::now();
    let mut last = 0;
    let mut first = 0;
    let mut seen = HashSet::new();
    let mut r1 = 0;
    const C1: usize = 10828530;
    const C2: usize = 65899;
    loop {
        let mut r2 = r1 | 65536;
        r1 = C1;
        while r2 > 0 {
            r1 = (((r1 + (r2 & 255)) & 0xFFFFFF) * C2) & 0xFFFFFF;
            r2 = r2 >> 8;
        }
        // if r1 == r0 { return }
        if seen.is_empty() {
            first = r1;
        }
        if !seen.insert(r1) {
            break;
        }
        last = r1;
    }
    println!("elapsed: {:?}", start.elapsed());
    println!("part1: {}", first);
    println!("part2: {}", last);
}
