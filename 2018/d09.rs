struct Node<T> {
    val: T,
    prev: usize,
    next: usize,
}
struct List<T> {
    raw: Vec<Node<T>>,
    pos: usize,
}
impl<T> List<T> {
    fn new(val: T) -> List<T> {
        List {
            raw: vec![Node {
                val: val,
                prev: 0,
                next: 0,
            }],
            pos: 0,
        }
    }
    fn get(&self) -> &T {
        &self.raw[self.pos].val
    }
    fn forward(&mut self) {
        self.pos = self.raw[self.pos].next;
    }
    fn backward(&mut self) {
        self.pos = self.raw[self.pos].prev;
    }
    fn remove(&mut self) {
        let list = &mut self.raw;
        let (prev, next) = {
            let node = &list[self.pos];
            (node.prev, node.next)
        };
        list[prev].next = next;
        list[next].prev = prev;
        self.pos = next;
    }
    fn append(&mut self, val: T) {
        let list = &mut self.raw;
        let next = list[self.pos].next;
        list.push(Node {
            val: val,
            prev: self.pos,
            next: next,
        });
        let pos = list.len() - 1;
        list[self.pos].next = pos;
        list[next].prev = pos;
        self.pos = pos;
    }
}

#[allow(dead_code)]
fn print_list<T: std::fmt::Display>(cur: &mut List<T>) {
    print!("{}, ", cur.get());
    let start = cur.pos;
    cur.forward();
    while cur.pos != start {
        print!("{}, ", cur.get());
        cur.forward();
    }
    println!();
}

#[allow(dead_code)]
fn solve(nplayers: usize, last_marble: usize) -> usize {
    let mut players = vec![0; nplayers];
    let mut circle = List::<usize>::new(0);
    //let mut pos = 0;
    for m in 1..=last_marble {
        //print_list(start);
        if m % 23 == 0 {
            let playern = (m - 1) % nplayers;
            for _ in 0..7 { circle.backward(); }
            players[playern] += m + circle.get();
            circle.remove();
        } else {
            circle.forward();
            circle.append(m);
        }
    }
    return *players.iter().max().expect("at least one player required");
}

#[allow(dead_code)]
fn solve2(player_nb: usize, marble_number: usize ) -> usize{
    use std::collections::VecDeque;
    let mut players_score = vec!(0; player_nb);
    let mut ring = VecDeque::new();
    ring.push_front(0);

    for marble in 1..marble_number {
        if marble % 23 == 0{
            // rotate of 7 behind + delete
            for _ in 0..7 {
                let tmp = ring.pop_back().expect("Rotate problem");
                ring.push_front(tmp);
            }
            players_score[marble % player_nb] +=
                    marble + ring.pop_front().expect("No value in the ring");
        } else {
            for _ in 0..2 {
                let tmp = ring.pop_front().expect("Rotate problem");
                ring.push_back(tmp);
            }
            ring.push_front(marble);
        }
    }
    *players_score.iter().max().expect("No value in the player scores")
}

fn main() {
    const NPLAYERS: usize = 429;
    const LAST_MARBLE: usize = 70901;
    let start = std::time::Instant::now();
    let part1 = solve(NPLAYERS, LAST_MARBLE);
    let part2 = solve(NPLAYERS, LAST_MARBLE*100);
    let elapsed = start.elapsed();
    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("elapsed: {:?}", elapsed);
}
