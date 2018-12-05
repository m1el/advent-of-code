use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::VecDeque;
use std::time::Instant;
const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_C: usize = 2;
const REG_D: usize = 3;
const REG_E: usize = 4;
const REG_F: usize = 5;
const REG_G: usize = 6;
const REG_H: usize = 7;

#[derive(Clone, Copy, Debug)]
enum Inst {
    Sub(usize, RegVal),
    Set(usize, RegVal),
    Add(usize, RegVal),
    Mul(usize, RegVal),
    Mod(usize, RegVal),
    Jgz(RegVal, RegVal),
    Jnz(RegVal, RegVal),
    Rcv(usize),
    Snd(RegVal),
}
#[derive(Clone, Copy, Debug)]
enum RegVal {
    Reg(usize),
    Val(i64),
}
fn reg_idx(c: char) -> usize {
    match c {
        'a' => REG_A,
        'b' => REG_B,
        'c' => REG_C,
        'd' => REG_D,
        'e' => REG_E,
        'f' => REG_F,
        'g' => REG_G,
        'h' => REG_H,
        _ => panic!("unknown reg '{}'", c),
    }
}
impl RegVal {
    fn from_str(s: &str) -> RegVal {
        let c = s.chars().next().unwrap();
        if c.is_alphabetic() {
            RegVal::Reg(reg_idx(c))
        } else {
            RegVal::Val(s.parse::<i64>().unwrap())
        }
    }
}
#[derive(Debug)]
struct CPU {
    ip: i64,
    regs: [i64; 8],
    halt: bool,
    queue: VecDeque<i64>,
    waiting: bool,
    tsc: usize,
    mul: usize,
}
impl CPU {
    fn new(id: i64) -> CPU {
        CPU {
            ip: 0,
            regs: [0, 0, 0, 0, 0, 0, 0, 0],
            queue: VecDeque::new(),
            halt: false,
            waiting: false,
            mul: 0,
            tsc: 0,
        }
    }


    fn run_emu(&mut self, prog: &[Inst], oq: &mut VecDeque<i64>) {
        if self.halt {
            return;
        }
        use Inst::*;
        loop {
            if self.ip < 0 || self.ip as usize >= prog.len() {
                println!("cpu halt");
                self.halt = true;
                return;
            }
            let inst = prog[self.ip as usize];
            match inst {
                Set(r, rv) => self.regs[r] = self.lookup(rv),
                Sub(r, rv) => self.regs[r] -= self.lookup(rv),
                Add(r, rv) => self.regs[r] += self.lookup(rv),
                Mul(r, rv) => { self.mul += 1; self.regs[r] *= self.lookup(rv); },
                Mod(r, rv) => self.regs[r] %= self.lookup(rv),
                Jgz(cnd, off) => {
                    if self.lookup(cnd) > 0 {
                        self.ip += self.lookup(off);
                        self.tsc += 1;
                        continue;
                    }
                },
                Jnz(cnd, off) => {
                    if self.lookup(cnd) != 0 {
                        self.ip += self.lookup(off);
                        self.tsc += 1;
                        continue;
                    }
                },
                Snd(rv) => {
                    continue;
                    oq.push_front(self.lookup(rv));
                    //self.regs[REG_SENT] += 1;
                },
                Rcv(r) => {
                    continue;
                    if let Some(val) = self.queue.pop_back() {
                        //self.regs[REG_WAITING] = 0;
                        self.regs[r] = val;
                    } else {
                        //self.regs[REG_WAITING] = 1;
                        return
                    }
                },
            }
            self.ip += 1;
            self.tsc += 1;
        }
    }

    fn running(&self) -> bool {
        !self.halt
    }

    fn lookup(&self, rv: RegVal) -> i64 {
        match rv {
            RegVal::Reg(r) => self.regs[r],
            RegVal::Val(v) => v,
        }
    }
}

fn main() {
    let start = Instant::now();

    let instrs = {
        let f = File::open("23.txt").expect("coudn't open input file");
        let fd = BufReader::new(&f);
        let mut instrs = Vec::<Inst>::new();
        for line in fd.lines() {
            let l = line.expect("couldn't read line");
            let mut words = l.split_whitespace();
            let first = words.next().expect("empty instruction line");
            let arg1 = words.next().expect("no arg1");
            let arg2 = words.next();
            fn reg(s: &str) -> usize { reg_idx(s.chars().next().expect("empty register name")) }
            let i = match first {
                "snd" => Inst::Snd(RegVal::from_str(arg1)),
                "rcv" => Inst::Rcv(reg(arg1)),
                "set" => Inst::Set(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "add" => Inst::Add(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "sub" => Inst::Sub(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "mul" => Inst::Mul(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "mod" => Inst::Mod(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "jgz" => Inst::Jgz(RegVal::from_str(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "jnz" => Inst::Jnz(RegVal::from_str(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                _ => panic!("unespected instruction: {}", first),
            };
            instrs.push(i);
        }
        instrs
    };
    let parsing_time = start.elapsed();

    //let trans_start = Instant::now();
    //use Inst::*;
    //use RegVal::*;
    //let (translated, jump_table) = JitTranslator::translate(&instrs);
    //let fun = buf_to_fn(&translated);
    //let translation_time = trans_start.elapsed();

    let part1_start = Instant::now();
    let part1 = {
        let mut cpu1 = CPU::new(0);
        let mut q1 = VecDeque::new();
        cpu1.run_emu(&instrs, &mut q1);
        cpu1.mul
    };
    let part1_time = part1_start.elapsed();
    println!("{}", part1);
}

