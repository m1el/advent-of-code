struct RecipeIter {
    items: Vec<u8>,
    p1: usize,
    p2: usize,
    pos: usize,
}

impl RecipeIter {
    fn new() -> RecipeIter {
        RecipeIter {
            items: vec![3, 7],
            p1: 0,
            p2: 1,
            pos: 0,
        }
    }
}

impl Iterator for RecipeIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(it) = self.items.get(self.pos) {
                self.pos += 1;
                return Some(*it);
            }

            let mut score = self.items[self.p1] + self.items[self.p2];
            if score >= 10 {
                self.items.push(score / 10);
                score %= 10;
            }
            self.items.push(score);
            self.p1 += (self.items[self.p1] + 1) as usize;
            self.p1 %= self.items.len();
            self.p2 += (self.items[self.p2] + 1) as usize;
            self.p2 %= self.items.len();
        }
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input = 147061;
    let part1 = RecipeIter::new()
        .skip(input).take(10)
        .map(|d| (d + b'0') as char)
        .collect::<String>();

    let mut digits = 0;
    let mut part2 = 0;
    for (idx, n) in RecipeIter::new().enumerate() {
        digits = (digits % 100_000) * 10 + (n as usize);
        if digits == input {
            part2 = idx - 6;
            break;
        }
    }
    println!("elapsed: {:?}", start.elapsed());
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
