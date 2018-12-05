use std::time::{Instant};
use std::collections::{VecDeque};
use std::io::{BufReader,BufRead,Write};
use std::fs::{File};

#[derive(Clone, Copy, Debug)]
enum RegVal {
    Reg(usize),
    Val(i64),
}
fn reg_idx(c: char) -> usize {
    match c {
        'a' => REG_A,
        'b' => REG_B,
        'f' => REG_F,
        'i' => REG_I,
        'p' => REG_P,
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

#[derive(Clone, Copy, Debug)]
enum Inst {
    Dbg(RegVal),
    Set(usize, RegVal),
    Add(usize, RegVal),
    Mul(usize, RegVal),
    Mod(usize, RegVal),
    Jgz(RegVal, RegVal),
    Rcv(usize),
    Snd(RegVal),
}

const REG_A: usize = 0;
const REG_B: usize = 1;
const REG_F: usize = 2;
const REG_I: usize = 3;
const REG_P: usize = 4;
const REG_WAITING: usize = 5;
const REG_SENT: usize = 6;
const REG_IP: usize = 7;

fn buf_to_fn(buf: &[u8]) -> TranslatedFn {
    const MEM_COMMIT:  u32 = 0x00001000;
    //const MEM_RESERVE: u32 = 0x00002000;
    const PAGE_READWRITE:         u32 = 0x04;
    const PAGE_EXECUTE_READ:      u32 = 0x20;
    //const PAGE_EXECUTE_READWRITE: u32 = 0x40;
    extern "C" {
        fn VirtualAlloc(ptr: *mut u8, size: usize, typ: u32, protect: u32) -> *mut u8;
        fn VirtualProtect(ptr: *mut u8, size: usize, protect: u32, old_protect: &mut u32) -> u32;
    }
    unsafe {
        //const FOUR_KILO: usize = 1024*4;
        let size = buf.len();
        let ptr = VirtualAlloc(0 as *mut u8, size, MEM_COMMIT, PAGE_READWRITE);
        let slice: &mut [u8] = std::slice::from_raw_parts_mut(ptr, buf.len());
        slice.clone_from_slice(buf);
        let mut params = 0u32;
        let _rv = VirtualProtect(ptr, size, PAGE_EXECUTE_READ, &mut params);
        std::mem::transmute::<*mut u8, TranslatedFn>(ptr)
    }
}

fn print_hex(buf: &[u8]) {
    let mut s = String::new();
    use std::fmt::{Write};
    for &byte in buf {
        write!(&mut s, "{:02X}", byte).expect("Unable to write");
    }
    println!("{}", s);
}

#[repr(C)]
struct FnArg {
    jump_table: *const usize, // +0
    pushq: extern "C" fn(queue: &mut VecDeque<i64>, val: i64), // +0x08
    popq: extern "C" fn(queue: &mut VecDeque<i64>, waiting: &mut i64) -> i64, // +0x10
    debug: extern "C" fn(i: i64), // +0x18
    start: TranslatedFn, // +0x20
    iq: *mut VecDeque<i64>, // +0x28
    oq: *mut VecDeque<i64>, // +0x30
}

type TranslatedFn = extern "C" fn(regs: *mut i64, arg: &FnArg) -> u64;

#[derive(Debug)]
struct Fixup {
    byte_pos: usize,
    source: usize,
    destination: usize,
}
impl Fixup {
    fn new(byte_pos: usize, source: usize, destination: usize) -> Fixup {
        Fixup { byte_pos, source, destination }
    }
}

struct JitTranslator {
    pos: usize,
    total: usize,
    jump_table: Vec<usize>,
    fixup_table: Vec<Fixup>,
    buf: Vec<u8>,
}

impl JitTranslator {
    fn i32_as_bytes(x: i32) -> [u8; 4] {
        unsafe { std::mem::transmute(x) }
    }
    fn i64_as_bytes(x: i64) -> [u8; 8] {
        unsafe { std::mem::transmute(x) }
    }
    fn write_prelude(&mut self) {
        self.buf.write_all(&[
            //0xf4,
            0x53,                   // push %rbx
            0x55,                   // push %rbp
            0x56,                   // push %rsi
            0x57,                   // push %rdi
            0x51,                   // push %rcx
            0x41, 0x54,             // push %r12
            0x41, 0x55,             // push %r13
            0x41, 0x56,             // push %r14
            0x41, 0x57,             // push %r15
            0x41, 0x57,             // push %r15
            0x48, 0x89, 0xCB,       // mov %rbx, %rcx
            0x48, 0x89, 0xD5,       // mov %rbp, %rdx
            0x4C, 0x8B, 0x19,       // mov %r11, [%rcx]
            0x4C, 0x8B, 0x61, 0x08, // mov %r12, [%rcx + 8]
            0x4C, 0x8B, 0x69, 0x10, // mov %r13, [%rcx + 16]
            0x4C, 0x8B, 0x71, 0x18, // mov %r14, [%rcx + 24]
            0x4C, 0x8B, 0x79, 0x20, // mov %r15, [%rcx + 32]
            // continuation trampoline
            0x48, 0x8B, 0x41, 0x38, // mov %rax, [%rcx+0x38]
            0x48, 0x85, 0xC0,       // test %rax, %rax
            0x0F, 0x84, 0x02, 0x00, 0x00, 0x00, // jz +1
            0xFF, 0xE0, // jmp %rax
        ]).unwrap();
    }

    fn write_outro(&mut self) {
        self.buf.write_all(&[
            0x48, 0x89, 0xD9,       // mov %rcx, %rbx
            0x4C, 0x89, 0x19,       // mov [%rcx], %r11
            0x4C, 0x89, 0x61, 0x08, // mov [%rcx + 8], %r12
            0x4C, 0x89, 0x69, 0x10, // mov [%rcx + 16], %r13
            0x4C, 0x89, 0x71, 0x18, // mov [%rcx + 24], %r14
            0x4C, 0x89, 0x79, 0x20, // mov [%rcx + 32], %r15
            0x41, 0x5f,             // pop %r15
            0x41, 0x5f,             // pop %r15
            0x41, 0x5e,             // pop %r14
            0x41, 0x5d,             // pop %r13
            0x41, 0x5c,             // pop %r12
            0x59,                   // pop %rcx
            0x5f,                   // pop %rdi
            0x5e,                   // pop %rsi
            0x5d,                   // pop %rbp
            0x5b,                   // pop %rbx
            0xc3,                   // ret
        ]).unwrap();
    }
    fn write_mov_reg(&mut self, dst: usize, src: usize) {
        let dst = dst as u8;
        let src = src as u8;
        self.buf.push(0x4d);
        self.buf.push(0x89);
        self.buf.push(0xdb + src * 8 + dst);
    }
    fn write_mov_const(&mut self, dst: usize, val: i64) {
        let dst = dst as u8;
        self.buf.push(0x49);
        self.buf.push(0xbb + dst);
        self.buf.write_all(&JitTranslator::i64_as_bytes(val)).unwrap();
    }
    fn write_ret(&mut self) {
        let total = self.total;
        self.write_jmp_abs(total); // jump to the outro with actual ret
    }
    fn write_jmp_rel(&mut self, off: i64) {
        let pos = self.pos as i64;
        self.write_jmp_abs((pos + off) as usize);
    }
    fn write_jmp_abs(&mut self, to: usize) {
        self.buf.write_all(&[0xe9, 0x00, 0x00, 0x00, 0x00]).unwrap();
        self.fixup_table.push(Fixup::new(self.buf.len() - 4, self.buf.len(), to));
    }
    fn write_jgz_reg(&mut self, cnd: usize, off: usize) {
        let pos = self.pos * 8;
        let pl = (pos & 0xff) as u8;
        let ph = ((pos >> 8) & 0xff) as u8;
        let cnd = cnd as u8;
        let off = off as u8;
        self.buf.write_all(&[
            0x49, 0x83, 0xfb + cnd, 0x00, // cmp %r12, 0
            0x0f, 0x8e, 0x12, 0x00, 0x00, 0x00, // jle +3
            0x48, 0x8B, 0x45, 0x00, // mov %rax, [%rbp]
            0x4A, 0x8B, 0x84, 0xD8 + off * 8, pl, ph, 0x00, 0x00, // mov %rax, [%rax+8*%r12+off]
            0x48, 0x03, 0x45, 0x20, // add %rax, [%rbp+0x20]
            0xff, 0xe0, // jmp %rax
        ]).unwrap();
    }
    fn write_jgz_rel(&mut self, cnd: usize, off: i64) {
        let pos = self.pos as i64;
        self.write_jgz_abs(cnd, (pos + off) as usize);
    }
    fn write_jgz_abs(&mut self, cnd: usize, to: usize) {
        let cnd = cnd as u8;
        self.buf.write_all(&[
            0x49, 0x83, 0xfb + cnd, 0x00, // cmp %r12, 0
            0x0F, 0x8f, 0x00, 0x00, 0x00, 0x00, // jg offset
        ]).unwrap();
        self.fixup_table.push(Fixup::new(self.buf.len() - 4, self.buf.len(), to));
    }
    fn write_add_const(&mut self, dst: usize, val: i64) {
        let dst = dst as u8;
        self.buf.push(0x49);
        self.buf.push(0x81);
        self.buf.push(0xc3 + dst);
        self.buf.write_all(&JitTranslator::i32_as_bytes(val as i32)).unwrap();
    }
    fn write_add_reg(&mut self, dst: usize, src: usize) {
        let dst = dst as u8;
        let src = src as u8;
        self.buf.push(0x4d);
        self.buf.push(0x01);
        self.buf.push(0xdb + src * 8 + dst);
    }
    fn write_mul_const(&mut self, dst: usize, val: i64) {
        let dst = dst as u8;
        self.buf.push(0x4d);
        self.buf.push(0x69);
        self.buf.push(0xdb + dst * 9);
        self.buf.write_all(&JitTranslator::i32_as_bytes(val as i32)).unwrap();
    }
    fn write_mul_reg(&mut self, dst: usize, src: usize) {
        let dst = dst as u8;
        let src = src as u8;
        self.buf.write_all(&[
            0x4d, 0x0f, 0xaf, 0xdb + src * 8 + dst // mul
        ]).unwrap();
    }
    fn write_mod_const(&mut self, dst: usize, val: i64) {
        let dst = dst as u8;
        self.buf.write_all(&[
            0x4c, 0x89, 0xd8 + dst * 8, // mov rax, r11
            0x48, 0xb9,    // mov rcx, val
        ]).unwrap();
        self.buf.write_all(&JitTranslator::i64_as_bytes(val)).unwrap();
        self.buf.write_all(&[
            0x48, 0x31, 0xD2, // xor rdx, rdx
            0x48, 0xF7, 0xF9, // idiv rcx
            0x49, 0x89, 0xD3 + dst, // mov r11, rdx
        ]).unwrap();
    }
    fn write_mod_reg(&mut self, dst: usize, src: usize) {
        let dst = dst as u8;
        let src = src as u8;
        self.buf.write_all(&[
            0x4C, 0x89, 0xD8 + dst * 8, // mov rax, r11
            0x48, 0x31, 0xD2,           // xor rdx, rdx
            0x49, 0xF7, 0xFB + src,     // idiv r11
            0x49, 0x89, 0xD3 + dst,     // mov r11, rdx
        ]).unwrap();
    }
    fn write_recv_reg(&mut self, reg: usize) {
        self.buf.write_all(&[
            0x48, 0x8B, 0x4D, 0x28, // mov %rcx, [%rbp+0x28]
            0x48, 0x8D, 0x53, 0x28, // lea %rdx, [%rbx+0x28]
            0x41, 0x53, // push %r11
            0xff, 0x55, 0x10, // call [%rbp+16]
            0x41, 0x5B, // pop %r11
            0x4C, 0x8B, 0x53, 0x28, // mov %r10, [%rbx+0x28]
            0x4D, 0x85, 0xD2, // test %r10, %r10
            0x0F, 0x84, 0x10, 0x00, 0x00, 0x00, // jz +3
            0x48, 0x8D, 0x05, 0xDD, 0xFF, 0xFF, 0xFF, // lea %rax, [%rip-0x23]
            0x48, 0x89, 0x43, 0x38, // mov [%rbx+0x38], %rax
            0xe9, 0x00, 0x00, 0x00, 0x00, // jmp return
            0x49, 0x89, 0xc3 + (reg as u8),
        ]).unwrap();
        self.fixup_table.push(Fixup::new(self.buf.len() - 7, self.buf.len() - 3, self.total));
    }
    fn write_send_reg(&mut self, reg: usize) {
        self.buf.write_all(&[
            0x48, 0x8B, 0x4D, 0x30, // mov %rcx, [%rbp+0x30]
            0x4c, 0x89, 0xda + (reg as u8) * 8, // mov %rdx, %r12
            0x48, 0xFF, 0x43, 0x30, // incq [%rbx+0x30]
            0x41, 0x53, // push %r11
            0xff, 0x55, 0x08, // call [%rbp+8]
            0x41, 0x5B, // pop %r11
        ]).unwrap();
    }
    fn _write_recv_reg(&mut self, reg: usize) {
        let reg = reg as u8;
        self.buf.write_all(&[
            0x48, 0x8B, 0x43, 0x28, // mov %rax, [%rbx+0x28]
            0x49, 0x89, 0xC3 + reg, // mov r11, rax
            0x48, 0x83, 0xF8, 0x00, // cmp %rax, 0
            0x0F, 0x85, 0x00, 0x00, 0x00, 0x00, // jnz return
        ]).unwrap();
        self.fixup_table.push(Fixup::new(self.buf.len() - 4, self.buf.len(), self.total));
    }
    fn _write_send_reg(&mut self, reg: usize) {
        let reg = reg as u8;
        let buf: &mut[u8] = &mut [
            0x4C, 0x89, 0x5b + reg * 8, 0x28, // mov [%rbx+28], %r11
        ];
        self.buf.write_all(buf).unwrap();
    }
    fn write_dbg_const(&mut self, val: i64) {
        let buf: &mut[u8] = &mut [
            0x48, 0xb9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov %rcx, val
            0x41, 0x53, // push %r11
            0xff, 0x55, 0x18, // call [%rbp+24]
            0x41, 0x5B, // pop %r11
        ];
        buf[2..10].clone_from_slice(&JitTranslator::i64_as_bytes(val));
        self.buf.write_all(buf).unwrap();
    }
    fn write_dbg_reg(&mut self, reg: usize) {
        let reg = reg as u8;
        let buf: &mut[u8] = &mut [
            0x4c, 0x89, 0xd9 + reg * 8, // mov %rcx, val
            0x41, 0x53, // push %r11
            0xff, 0x55, 0x18, // call [%rbp+24]
            0x41, 0x5B, // pop %r11
        ];
        self.buf.write_all(buf).unwrap();
    }
    fn write_dbg_regs(&mut self) {
        let buf: &mut[u8] = &mut [
            0x48, 0x89, 0xD9,       // mov %rcx, %rbx
            0x4C, 0x89, 0x19,       // mov [%rcx], %r11
            0x4C, 0x89, 0x61, 0x08, // mov [%rcx + 8], %r12
            0x4C, 0x89, 0x69, 0x10, // mov [%rcx + 16], %r13
            0x4C, 0x89, 0x71, 0x18, // mov [%rcx + 24], %r14
            0x4C, 0x89, 0x79, 0x20, // mov [%rcx + 32], %r15
            0x41, 0x53, // push %r11
            0xff, 0x55, 0x18, // call [%rbp+24]
            0x41, 0x5B, // pop %r11
        ];
        self.buf.write_all(buf).unwrap();
    }
    fn insert_jump_location(&mut self) {
        self.jump_table.push(self.buf.len());
    }
    fn translate(prog: &[Inst]) -> (Vec<u8>, Vec<usize>) {
        let mut translator = JitTranslator {
            pos: 0,
            total: prog.len(),
            jump_table: Vec::new(),
            fixup_table: Vec::new(),
            buf: Vec::new(),
        };

        translator.write_prelude();
        translator.insert_jump_location();
        for (idx, &inst) in prog.iter().enumerate() {
            translator.pos = idx;
            use Inst::*;
            use RegVal::*;
            match inst {
                Set(dst, src) => {
                    match src {
                        Val(v) => translator.write_mov_const(dst, v),
                        Reg(r) => translator.write_mov_reg(dst, r),
                    }
                },
                Add(dst, src) => {
                    match src {
                        Val(v) => translator.write_add_const(dst, v),
                        Reg(r) => translator.write_add_reg(dst, r),
                    }
                },
                Mul(dst, src) => {
                    match src {
                        Val(v) => translator.write_mul_const(dst, v),
                        Reg(r) => translator.write_mul_reg(dst, r),
                    }
                },
                Mod(dst, src) => {
                    match src {
                        Val(v) => translator.write_mod_const(dst, v),
                        Reg(r) => translator.write_mod_reg(dst, r),
                    }
                },
                Jgz(dst, to) => {
                    match (dst, to) {
                        (Val(v), Val(off)) => if v > 0 {
                            translator.write_jmp_rel(off);
                        },
                        (Val(v), Reg(_off)) => if v > 0 {
                            //translator.write_jmp_reg(off);
                        }
                        (Reg(cnd), Val(to)) => translator.write_jgz_rel(cnd, to),
                        (Reg(cnd), Reg(to)) => translator.write_jgz_reg(cnd, to),
                    }
                },
                Rcv(dst) => translator.write_recv_reg(dst),
                Snd(v) => {
                    match v {
                        Val(_v) => {}, //translator.write_send_const(v),
                        Reg(r) => translator.write_send_reg(r),
                    }
                },
                Dbg(v) => {
                    match v {
                        Val(v) => translator.write_dbg_const(v),
                        Reg(r) => translator.write_dbg_reg(r),
                    }
                }
            }
            translator.insert_jump_location();
        }
        translator.write_outro();

        for fixup in translator.fixup_table {
            let dst = translator.jump_table[fixup.destination] as i32;
            let off = dst - (fixup.source as i32);
            let slice = &mut translator.buf[fixup.byte_pos..fixup.byte_pos+4];
            slice.clone_from_slice(&JitTranslator::i32_as_bytes(off));
        }

        (translator.buf, translator.jump_table)
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
}

extern "C"
fn debug(i: i64) {
    println!("debug: {}", i);
}

extern "C"
fn pop_q(queue: &mut VecDeque<i64>, waiting: &mut i64) -> i64 {
    if let Some(val) = queue.pop_back() {
        *waiting = 0;
        return val;
    } else {
        *waiting = 1;
        return 0;
    }
}

extern "C"
fn push_q(queue: &mut VecDeque<i64>, val: i64) {
    //println!("push {}", val);
    queue.push_front(val);
}

impl CPU {
    fn new(id: i64) -> CPU {
        CPU {
            ip: 0,
            regs: [0, 0, 0, 0, id, 0, 0, 0],
            queue: VecDeque::new(),
            halt: false,
            waiting: false,
            tsc: 0,
        }
    }


    fn run_cpu(&mut self, compiled: TranslatedFn, jump_table: &[usize], oq: &mut VecDeque<i64>) {
        let iq = &mut self.queue;
        let arg = FnArg {
            jump_table: jump_table.as_ptr(),
            start: compiled,
            pushq: push_q,
            popq: pop_q,
            debug: debug,
            iq: iq as *mut _,
            oq: oq as *mut _,
        };
        let v = compiled((&mut self.regs).as_mut_ptr(), &arg);
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
                Add(r, rv) => self.regs[r] += self.lookup(rv),
                Mul(r, rv) => self.regs[r] *= self.lookup(rv),
                Mod(r, rv) => self.regs[r] %= self.lookup(rv),
                Jgz(cnd, off) => {
                    if self.lookup(cnd) > 0 {
                        self.ip += self.lookup(off);
                        self.tsc += 1;
                        continue;
                    }
                },
                Snd(rv) => {
                    oq.push_front(self.lookup(rv));
                    self.regs[REG_SENT] += 1;
                },
                Rcv(r) => {
                    if let Some(val) = self.queue.pop_back() {
                        self.regs[REG_WAITING] = 0;
                        self.regs[r] = val;
                    } else {
                        self.regs[REG_WAITING] = 1;
                        return
                    }
                },
                Dbg(_) => {},
            }
            self.ip += 1;
            self.tsc += 1;
        }
    }

    fn running(&self) -> bool {
        !self.halt && !(self.regs[REG_WAITING] != 0 && self.queue.is_empty())
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
        let f = File::open("18.txt").expect("coudn't open input file");
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
                "mul" => Inst::Mul(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "mod" => Inst::Mod(reg(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "jgz" => Inst::Jgz(RegVal::from_str(arg1), RegVal::from_str(arg2.expect("no arg2"))),
                "dbg" => Inst::Dbg(RegVal::from_str(arg1)),
                _ => panic!("unespected instruction: {}", first),
            };
            instrs.push(i);
        }
        instrs
    };
    let parsing_time = start.elapsed();

    let trans_start = Instant::now();
    use Inst::*;
    use RegVal::*;
    let (translated, jump_table) = JitTranslator::translate(&instrs);
    let fun = buf_to_fn(&translated);
    let translation_time = trans_start.elapsed();

    let part1_start = Instant::now();
    let part1 = {
        //let mut cpu0 = CPU::new(0);
        let mut cpu1 = CPU::new(0);
        //let mut q0 = VecDeque::new();
        let mut q1 = VecDeque::new();
        //cpu0.run_emu(&instrs, &mut q0);
        cpu1.run_cpu(fun, &jump_table, &mut q1);
        //println!("cpu0: {:?}", cpu0);
        //println!("queue: {:?}", q0);
        q1.pop_front().unwrap()
    };
    let part1_time = part1_start.elapsed();

    let part2_start = Instant::now();
    let mut cpu0 = CPU::new(0);
    let mut cpu1 = CPU::new(1);
    while cpu0.running() || cpu1.running() {
        cpu0.run_cpu(fun, &jump_table, &mut cpu1.queue);
        cpu1.run_cpu(fun, &jump_table, &mut cpu0.queue);
        //cpu0.run_emu(&instrs, &mut cpu1.queue);
        //cpu1.run_emu(&instrs, &mut cpu0.queue);
    }
    let part2_time = part2_start.elapsed();
    let total_time = start.elapsed();
    println!("parsing time: {:?}\ntranslation_time: {:?}\npart1 time: {:?}\npart2 time: {:?}\ntotal time: {:?}",
             parsing_time, translation_time, part1_time, part2_time, total_time);
    println!("part1 answer: {}", part1);
    println!("part2 answer: {}", cpu1.regs[REG_SENT]);
    println!("hex of the binary code:");
    print_hex(&translated);
}
