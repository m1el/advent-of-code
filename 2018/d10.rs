use std::ops::{AddAssign, Add, Mul, Div};
use std::fs::{File};
use std::io::{BufReader, BufRead};
#[derive(Clone, Copy, Debug)]
struct Pos(isize, isize);
impl Pos {
    fn sq(self: &Pos) -> Pos {
        Pos(self.0.pow(2), self.1.pow(2))
    }
    fn max(self: &Pos, other: &Pos) -> Pos {
        Pos(self.0.max(other.0), self.1.max(other.1))
    }
    fn min(self: &Pos, other: &Pos) -> Pos {
        Pos(self.0.min(other.0), self.1.min(other.1))
    }
}
impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}
impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, other: Pos) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl Div<isize> for Pos {
    type Output = Pos;

    fn div(self, other: isize) -> Pos {
        Pos(self.0 / other, self.1 / other)
    }
}
impl Mul<isize> for Pos {
    type Output = Pos;
    fn mul(self, other: isize) -> Pos {
        Pos(self.0 * other, self.1 * other)
    }
}
fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("10.txt")?;
    let reader = BufReader::new(fd);
    let mut inp = Vec::<(Pos, Pos)>::new();
    for line in reader.lines().filter_map(Result::ok) {
        let px = line[10..16].trim().parse().unwrap();
        let py = line[18..24].trim().parse().unwrap();
        let vx = line[36..38].trim().parse().unwrap();
        let vy = line[39..42].trim().parse().unwrap();
        inp.push((Pos(px, py), Pos(vx, vy)));
    }
    let npoints = inp.len() as isize;
    for step in 0..10369 {
        let mut sum = Pos(0,0);
        let mut sqsum = Pos(0,0);
        let mut max = None;
        let mut min = None;
        for &(p, v) in inp.iter() {
            let pp = p + v * step;
            max = max.or_else(||Some(pp)).map(|v| v.max(&pp));
            min = min.or_else(||Some(pp)).map(|v| v.min(&pp));
            sum += pp;
            sqsum += Pos(pp.0.pow(2), pp.1.pow(2));
        }
        sum = sum.sq();
        sqsum = sqsum * npoints;
        let diff = sqsum + (sum * - 1);
        println!("{} {:?} {:?} {:?}", step, diff, max, min);
    }
    for step in 10369..10384 {
        let mut field = vec![b' ';400*400];
        for &(p, v) in inp.iter() {
            let pp = p + v * step;
            field[(pp.0 + pp.1 * 400) as usize] = b'#';
        }
        println!("{}", step);
        for row in field.chunks(400) {
            let row = std::str::from_utf8(row).unwrap();
            println!("{}", row);
        }
    }
    Ok(())
}
