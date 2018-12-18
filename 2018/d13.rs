use std::collections::{BTreeMap};
use std::io::{BufReader, BufRead};
use std::fs::{File};

#[derive(Clone,Copy,Debug)]
enum Dir {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Dir {
    fn from_u8(ch: u8) -> Self {
        match ch {
            b'^' => Dir::Up,
            b'v' => Dir::Down,
            b'>' => Dir::Right,
            b'<' => Dir::Left,
            _ => unreachable!(),
        }
    }
    fn from_num(num: usize) -> Self {
        match num {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => unreachable!(),
        }
    }
    fn drive(&self, xy: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (xy.0, xy.1-1),
            Dir::Down => (xy.0, xy.1+1),
            Dir::Left => (xy.0-1, xy.1),
            Dir::Right => (xy.0+1, xy.1),
        }
    }
    fn apply(&self, turn: Turn) -> Self {
        Dir::from_num(((*self as usize) + (turn as usize)) % 4)
    }
}

#[derive(Clone,Copy,Debug)]
enum Turn {
    Left = 3,
    Straight = 0,
    Right = 1,
}

impl Turn {
    fn next(self) -> Self {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Clone,Copy,Debug)]
struct Cart {
    dead: bool,
    pos: (usize, usize),
    dir: Dir,
    state: Turn,
}
impl Cart {
    fn step(&mut self, track: u8) {
        self.dir = match track {
            b'/' => Dir::from_num((5 - (self.dir as usize)) % 4),
            b'\\' => Dir::from_num((7 - (self.dir as usize)) % 4),
            b'+' => {
                let dir = self.dir.apply(self.state);
                self.state = self.state.next();
                dir
            },
            _ => self.dir,
        };
        self.pos = self.dir.drive(self.pos);
    }
}

fn cart_track(cart: u8) -> u8 {
    match cart {
        b'^' | b'v' => b'|',
        b'>' | b'<' => b'-',
        _ => unreachable!(),
    }
}

const CARTS: &[u8] = &[b'^', b'v', b'<', b'>'];

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("13.txt")?;
    let reader = BufReader::new(fd);
    let mut grid: Vec<Vec<u8>> =
        reader.lines().filter_map(Result::ok)
              .map(|l| l.into_bytes())
              .collect();
    let start = std::time::Instant::now();
    let mut carts = Vec::new();
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, ch) in row.iter_mut().enumerate() {
            if CARTS.contains(ch) {
                carts.push(Cart {
                    dead: false,
                    pos: (x,y),
                    dir: Dir::from_u8(*ch),
                    state: Turn::Left,
                });
                *ch = cart_track(*ch);
            }
        }
    }
    let mut steps = 0;
    while carts.len() > 1 {
        steps += 1;
        carts.sort_by_key(|cart| (cart.pos.1, cart.pos.0));
        let mut visited: BTreeMap<(usize,usize),usize> =
                carts.iter().enumerate()
                .map(|(id, cart)| (cart.pos, id))
                .collect();
        let mut to_remove = Vec::new();
        for (id, cart) in carts.iter_mut().enumerate() {
            let (x, y) = cart.pos;
            visited.remove(&cart.pos);
            cart.step(grid[y][x]);
            if let Some(prev) = visited.insert(cart.pos, id) {
                to_remove.push(id);
                to_remove.push(prev);
                println!("remove pos: {:?}", cart.pos);
                visited.remove(&cart.pos);
            }
        }
        for id in to_remove {
            carts[id].dead = true;
        }
        carts.retain(|cart| !cart.dead);
    }
    println!("{:?}", carts);
    println!("elapsed: {:?}", start.elapsed());
    println!("steps: {:?}", steps);
    Ok(())
}
