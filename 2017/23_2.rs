#![feature(iterator_step_by)]
fn primeq(x: i64) -> bool {
    for i in 2..x/2 {
        if x % i == 0 { return false; }
    }
    return true;
}
fn main() {
    let mut b: i64 = 79*100 + 100000;
    let mut c = b + 17000;
    println!("{}", (b..c+1).step_by(17).filter(|x| !primeq(*x)).count());
}
