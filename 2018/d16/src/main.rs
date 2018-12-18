extern crate regex;

use std::fs::{File};
use std::io::{BufReader, BufRead};
use regex::Regex;

type Regs = [isize; 4];
type Perm = [usize; 16];
#[derive(Debug,Clone)]
struct OpCode {
    code: isize,
    a: isize,
    b: isize,
    c: isize,
}
fn eval(perm: &Perm, op: &OpCode, regs: &mut Regs) {
    let code = perm[op.code as usize];
    regs[op.c as usize] = match code {
        0x0 => regs[op.a as usize] + regs[op.b as usize],
        0x1 => regs[op.a as usize] + op.b,
        0x2 => regs[op.a as usize] * regs[op.b as usize],
        0x3 => regs[op.a as usize] * op.b,
        0x4 => regs[op.a as usize] & regs[op.b as usize],
        0x5 => regs[op.a as usize] & op.b,
        0x6 => regs[op.a as usize] | regs[op.b as usize],
        0x7 => regs[op.a as usize] | op.b,
        0x8 => regs[op.a as usize],
        0x9 => op.a,
        0xA => (op.a > regs[op.b as usize]) as isize,
        0xB => (regs[op.a as usize] > op.b) as isize,
        0xC => (regs[op.a as usize] > regs[op.b as usize]) as isize,
        0xD => (op.a == regs[op.b as usize]) as isize,
        0xE => (regs[op.a as usize] == op.b) as isize,
        0xF => (regs[op.a as usize] == regs[op.b as usize]) as isize,
        _ => unreachable!()
    }
}

fn line_to_4(line: String) -> Result<[isize; 4],Box<std::error::Error>> {
    let re = Regex::new(r"\d+")?;
    let mut rv = [0_isize; 4];
    for (i, m) in re.find_iter(&line).enumerate().take(4) {
        rv[i] = m.as_str().parse()?;
    }
    Ok(rv)
}

#[derive(Debug,Clone)]
struct Test {
    before: Regs,
    op: OpCode,
    after: Regs,
}

#[derive(Debug)]
struct NoSolution {}
impl std::fmt::Display for NoSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for NoSolution {}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("16.txt")?;
    let reader = BufReader::new(fd);
    let mut lines = reader.lines();
    let mut tests = Vec::new();
    loop {
        let b = lines.next().unwrap()?;
        if b.len() == 0 { break }
        let before = line_to_4(b)?;
        let op = line_to_4(lines.next().unwrap()?)?;
        let after = line_to_4(lines.next().unwrap()?)?;
        tests.push(Test {
            before: before,
            op: OpCode {
                code: op[0],
                a: op[1],
                b: op[2],
                c: op[3],
            },
            after: after,
        });
        lines.next().unwrap()?;
    }

    let count = &mut [0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];

    let mut good_test = 0;
    let mut possible_ops = vec![[true;16];16];
    for mut test in tests.iter().cloned() {
        let mut good_op = 0;
        let tmp = test.op.code;
        for op in 0x0..0x10 {
            test.op.code = op;
            let mut regs = test.before;
            eval(count, &test.op, &mut regs);
            if regs == test.after {
                good_op += 1;
            } else {
                possible_ops[tmp as usize][op as usize] = false;
            }
        }
        if good_op >= 3 {
            good_test += 1;
        }
    }

    println!("part1: {}", good_test);

    // solve permutation
    fn id<T>(x:T)->T{x}
    let mut found = [255; 16];
    'outer: while !found.iter().all(|&x| x<255) {
        for idx in 0..16 {
            if found[idx] != 255 { continue }
            let first = possible_ops[idx].iter().cloned().position(id).ok_or(NoSolution{})?;
            let last = possible_ops[idx].iter().cloned().rposition(id).ok_or(NoSolution{})?;
            if first != last { continue }
            found[idx] = first;
            for i in 0..16 {
                if idx == i { continue }
                possible_ops[i][first] = false;
            }
            continue 'outer;
        }

        return Err(Box::new(NoSolution{}));
    }

    println!("permutation: {:?}", found);

    let mut regs = [0; 4];
    for line in lines.filter_map(Result::ok) {
        let op = line_to_4(line)?;
        let op = OpCode {
            code: op[0],
            a: op[1],
            b: op[2],
            c: op[3],
        };
        eval(&found, &op, &mut regs);
    }

    println!("part2: {:?}", regs[0]);
    Ok(())
}
