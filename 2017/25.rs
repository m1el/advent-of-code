use std::time::{Instant};
use std::collections::{VecDeque};

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

fn main() {
    let start_time = Instant::now();
    use State::*;
    let mut state = A;
    let mut pos = 0isize;
    let mut off = 0isize;
    let mut tape = VecDeque::<u8>::new();
    let num = 3000; //12368930;
    for i in 0..num {
        let s: usize = tape.iter().map(|v|*v as usize).sum();
        println!("i: {}, sum: {}", i, s);
        while pos + off < 0 {
            tape.push_front(0);
            off += 1;
        }
        while (pos + off) as usize >= tape.len() {
            tape.push_back(0);
        }
        let current = tape.get_mut((pos + off) as usize)
                        .expect("tape overflow error");
        const L: isize = -1;
        const R: isize = 1;
        let (write, d, ns) = match (state, *current) {
            (A, 0) => (1, R, B),
            (A, 1) => (0, R, C),
            (B, 0) => (0, L, A),
            (B, 1) => (0, R, D),
            (C, 0) => (1, R, D),
            (C, 1) => (1, R, A),
            (D, 0) => (1, L, E),
            (D, 1) => (0, L, D),
            (E, 0) => (1, R, F),
            (E, 1) => (1, L, B),
            (F, 0) => (1, R, A),
            (F, 1) => (1, R, E),
            _ => panic!("wrong state?"),
        };
        *current = write;
        pos += d;
        state = ns;
    }
    let s: usize = tape.iter().map(|v|*v as usize).sum();
    let solution_time = start_time.elapsed();
    println!("pos: {}\noff: {}", pos, off);
    println!("answer: {}", s);
    println!("solution time: {:?}", solution_time);
}
