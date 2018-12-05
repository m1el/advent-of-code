const MODULUS: u64 = 0x7fffffff;
struct Generator {
    state: u64,
    mul: u64,
}

impl Generator {
    pub fn new(state: u64, mul: u64) -> Generator {
        Generator { state, mul }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.state = (self.state * self.mul) % MODULUS;
        Some(self.state)
    }
}

fn main() {
    let gen_a = Generator::new(634, 16807);
    let gen_b = Generator::new(301, 48271);
    let score = gen_a.zip(gen_b).take(40_000_000)
        .filter(|&(a, b)| a & 0xffff == b & 0xffff).count();

    println!("{}", score);

    let gen_a = Generator::new(634, 16807).filter(|a| a & 3 == 0);
    let gen_b = Generator::new(301, 48271).filter(|b| b & 7 == 0);
    let score = gen_a.zip(gen_b).take(5_000_000)
        .filter(|&(a, b)| a & 0xffff == b & 0xffff).count();

    println!("{}", score);
}
