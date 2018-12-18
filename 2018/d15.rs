use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::collections::{BTreeMap};
#[derive(Debug,Clone,Copy,PartialEq)]
enum UnitKind {
    Elf,
    Goblin
}
fn kind(ch: u8) -> UnitKind {
    match ch {
        b'E' => UnitKind::Elf,
        b'G' => UnitKind::Goblin,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Unit {
    kind: UnitKind,
    pos: (usize, usize),
    hp: isize,
}

fn neighbors(w: usize, h: usize, pos: (usize,usize)) -> Vec<(usize, usize)> {
        let (y,x) = pos;
        let mut next = vec![];
        if y > 0   { next.push((y-1,x)); }
        if x > 0   { next.push((y,x-1)); }
        if x < w-1 { next.push((y,x+1)); }
        if y < h-1 { next.push((y+1,x)); }
        next
}
fn find_path(unit: &mut Unit, grid: &mut Vec<Vec<u8>>) -> bool {
    let w = grid[0].len();
    let h = grid.len();
    let mut visited = BTreeMap::new();
    let mut queue = BTreeMap::new();
    let me = match unit.kind {
        UnitKind::Elf => b'E',
        UnitKind::Goblin => b'G',
    };
    let enemy = match unit.kind {
        UnitKind::Elf => b'G',
        UnitKind::Goblin => b'E',
    };
    queue.insert(unit.pos, 0);
    let mut target = None;
    'outer: while !queue.is_empty() {
        let (pos, it) = queue.iter().map(|(pos,it)|(*pos,*it))
            .min_by_key(|&(pos,it)|(it,pos)).unwrap();
        visited.entry(pos).or_insert(it);
        queue.remove(&pos);
        for pos in neighbors(w,h,pos) {
            let ch = grid[pos.0][pos.1];
            if ch == enemy {
                target = Some((pos,it+1));
                break 'outer;
            }
            if ch == b'.' && !visited.contains_key(&pos) {
                visited.insert(pos, it+1);
                queue.entry(pos).or_insert(it+1);
            }
        }
    }
    //println!("type: {}, target: {:?}", me, target);
    if let Some((target, mut it)) = target {
        let mut pos = target;
        let mut path = vec![];
        'step: while pos != unit.pos {
            //println!("mv pos {:?}", pos);
            path.push(pos);
            it -= 1;
            for p in neighbors(w,h,pos) {
                if visited.get(&p) == Some(&it) {
                    pos = p;
                    continue 'step;
                }
            }
            unreachable!();
        }
        if let Some(mv) = path.pop() {
            if mv != target {
                let (y,x) = unit.pos; grid[y][x] = b'.';
                unit.pos = mv;
                let (y,x) = unit.pos; grid[y][x] = me;
            }
            if dist(unit.pos, target) == 1 {
                return true;
            }
        }
    }
    false
}
fn dist(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() +
     (a.1 as isize - b.1 as isize).abs()) as usize
}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("15.txt")?;
    let reader = BufReader::new(fd);
    let mut units = vec![];
    let mut grid = vec![];
    for (y, line) in reader.lines().filter_map(Result::ok).enumerate() {
        let mut line = line.trim().to_string().into_bytes();
        for x in 0..line.len() {
            let ch = line[x];
            match ch {
                b'E' | b'G' => {
                    units.push(Unit {
                        kind: kind(ch),
                        pos: (y, x),
                        hp: 200,
                    });
                },
                _ => {},
            }
        }
        grid.push(line);
    }
    let mut rounds = 0;

    loop {
        let nelves = units.iter().filter(|unit| unit.kind == UnitKind::Elf).count();
        let ngoblins = units.iter().filter(|unit| unit.kind == UnitKind::Goblin).count();
        if nelves == 0 || ngoblins == 0 { break; }
        units.sort_by_key(|unit|unit.pos);
        for idx in 0..units.len() {
            if units[idx].hp <= 0 { continue }
            if find_path(&mut units[idx], &mut grid) {
                let src = units[idx].pos;
                let k = units[idx].kind;
                let mut min = (1000, (0,0), 0);
                for dst in 0..units.len() {
                    let unit = &units[dst];
                    if dist(src,unit.pos) != 1 || unit.hp <= 0 || k == unit.kind { continue }
                    let tup = (unit.hp, unit.pos, dst);
                    if min > tup {
                        min = tup;
                    }
                }
                let unit = &mut units[min.2];
                //println!("attack by {:?}{:?} at {:?}{:?}", k, src, unit.kind, unit.pos);
                let (y,x) = unit.pos;
                unit.hp -= match k {
                    UnitKind::Elf => 3,
                    UnitKind::Goblin => 3,
                };
                if unit.hp <= 0 {
                    //println!("unit death at {:?}", unit.pos);
                    grid[y][x] = b'.';
                }
            }
        }
        units.retain(|unit| unit.hp > 0);
        //for row in grid.iter() {
        //    println!("{}", unsafe { std::str::from_utf8_unchecked(row) });
        //}
        //println!("{:?}", units);
        //println!("------------------------------------------");
        rounds += 1;
    }
    println!("{:?}", units);
    println!("{}", units.len());
    rounds -= 1;
    let total_hp = units.iter().map(|unit| unit.hp).sum::<isize>();
    println!("{} {} {} {:?}", rounds, total_hp, units.len(), rounds * total_hp);
    Ok(())
}
