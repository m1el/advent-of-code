use std::time::{Instant};
use std::collections::{HashMap};
use std::io::{BufReader, BufRead};
use std::fs::{File};
type InputData = Vec<(usize, usize)>;
type AvailableMap = HashMap<usize, Vec<usize>>;
struct Solution<'a> {
    data: &'a InputData,
    avail: &'a AvailableMap,
    max_str: usize,
    max_len: usize,
    max_len_str: usize,
}
impl<'a> Solution<'a> {
    fn new(data: &'a InputData, avail: &'a AvailableMap) -> Solution<'a> {
        Solution {
            data: data,
            avail: avail,
            max_str: 0,
            max_len: 0,
            max_len_str: 0,
        }
    }
    fn solve(&mut self) -> (usize, usize) {
        let mut v = Vec::new();
        self.solve_inner(0, &mut v);
        return (self.max_str, self.max_len_str)
    }
    fn solve_inner(&mut self, num: usize, stack: &mut Vec<usize>) {
        self.add_score(stack);
        let avail = self.avail;
        if let Some(next) = avail.get(&num) {
            for i in next.iter() {
                if stack.contains(i) { continue; }
                let (mut a, mut b) = self.data[*i];
                if b == num { std::mem::swap(&mut a, &mut b); }
                stack.push(*i);
                self.solve_inner(b, stack);
                stack.pop();
            }
        }
    }
    fn add_score(&mut self, stack: &[usize]) {
        let len = stack.len();
        let score = stack.iter()
            .map(|&i| { let (a,b) = self.data[i]; a + b })
            .sum();
        if score > self.max_str { self.max_str = score; }
        if len == self.max_len && score > self.max_len_str {
            self.max_len_str = score;
        }
        if len > self.max_len {
            self.max_len = len;
            self.max_len_str = score;
        }
    }
}
fn main() {
    let start = Instant::now();
    let parse_time;
    let (data, avail) = {
        let mut data: InputData = Vec::new();
        let mut avail: AvailableMap = HashMap::new();
        let f = File::open("24.txt").expect("coudln't open input file");
        let fd = BufReader::new(&f);
        for (i, line) in fd.lines().enumerate() {
            let line = line.expect("couldn't read input line");
            let mut sp = line.split('/');
            let a = sp.next().expect("empty input line")
                      .parse::<usize>().expect("input is not an int");
            let b = sp.next().expect("expected two numbers")
                      .parse::<usize>().expect("input is not an int");
            data.push((a, b));
            avail.entry(a).or_insert_with(|| Vec::new()).push(i);
            avail.entry(b).or_insert_with(|| Vec::new()).push(i);
        }
        parse_time = start.elapsed();
        (data, avail)
    };
    let sol_start = Instant::now();
    let mut sol = Solution::new(&data, &avail);
    let (p1, p2) = sol.solve();
    let sol_time = sol_start.elapsed();
    println!("part1: {}, part2: {}", p1, p2);
    println!("parse time: {:?}\nsolution time: {:?}", parse_time, sol_time);
}
