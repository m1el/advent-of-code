use std::collections::{HashMap};
use std::io::{BufRead, BufReader};
use std::fs::{File};
use std::ops::{AddAssign};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct V3 {
    x: i64,
    y: i64,
    z: i64,
}

impl V3 {
    fn zero() -> V3 {
        V3 { x: 0, y: 0, z: 0 }
    }
    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
impl AddAssign for V3 {
    fn add_assign(&mut self, other: V3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
#[derive(Debug, Clone)]
struct P {
    id: usize,
    p: V3,
    v: V3,
    a: V3,
}
impl P {
    fn step(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

fn main() {
    let f = File::open("20.txt").expect("coudn't open input file");
    let fd = BufReader::new(&f);
    let mut particles = Vec::<P>::new();
    for (id, line) in fd.lines().enumerate() {
        let mut p = [V3::zero(); 3];
        for (idx, vec) in line.expect("couldn't read line").split(", ").enumerate() {
            let mut v = [0i64; 3];
            for (idx, val) in vec[3..vec.len()-1].split(",").enumerate() {
                v[idx] = val.parse().expect("couldn't parse int");
            }
            p[idx] = V3 { x: v[0], y: v[1], z: v[2] };
        }
        particles.push(P { id: id, p: p[0], v: p[1], a: p[2] });
    }
    let particles_copy = particles.clone();
    for _ in 0..500 {
        for p in particles.iter_mut() {
            p.step();
        }
    }
    particles.sort_by_key(|p| p.p.manhattan());

    println!("part 1: {:?}", particles[0].id);
    let mut particles = particles_copy;
    for _ in 0..500 {
        let mut occupied = HashMap::<V3,usize>::new();
        for p in particles.iter_mut() {
            p.step();
            *occupied.entry(p.p).or_insert(0) += 1;
        }
        particles.retain(|p| occupied[&p.p] == 1);
    }
    println!("part 2: {}", particles.len());
}
