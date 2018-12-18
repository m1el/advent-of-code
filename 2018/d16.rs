use std::fs::{File};
use std::io::{BufReader, BufRead};

struct Regs([u8; 4]);
struct Perm([u8; 16]);
struct OpCode {
    code: u8,
    a: u8,
    b: u8,
    c: u8,
}
fn eval(op: &OpCode, perm: &Perm, regs: &mut Regs) {
    let code = perm.0[op.code as usize];
    regs[op.c as usize] = match code {
        0x0 => regs[op.a as usize] + regs[op.b as usize],
        0x1 => regs[op.a] + op.b,
        0x2 => regs[op.a as usize] * regs[op.b as usize],
        0x3 => regs[op.a] * op.b,
        0x4 => regs[op.a as usize] & regs[op.b as usize],
        0x5 => regs[op.a] & op.b,
        0x6 => regs[op.a as usize] | regs[op.b as usize],
        0x7 => regs[op.a] | op.b,
        0x8 => regs[op.a as usize],
        0x9 => op.a,
        0xA => (op.a > regs[op.b as usize]) as u8,
        0xB => (regs[op.a as usize] > op.b) as u8,
        0xC => (regs[op.a as usize] > regs[op.b as usize]) as u8,
        0xD => (op.a == regs[op.b as usize]) as u8,
        0xE => (regs[op.a as usize] == op.b) as u8,
        0xF => (regs[op.a as usize] == regs[op.b as usize]) as u8,
        _ => unreachable!()
    }
}

fn main() -> Result<(), std::error::Error> {
    let fd = File::open("16.txt")?;
    let reader = BufReader::new(fd);
    let mut things = Vec::new();
    let mut lines = things.take(3091);
    loop {
    }
    for line in reader.lines().filter_map(Result::ok) {
        things.push(line.parse::<isize>()?);
    }

    Ok(())
}
