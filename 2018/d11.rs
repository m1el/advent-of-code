fn main() {
    fn powl(id: isize, x: isize, y: isize) -> isize {
      (((x+10)*y+id)*(x+10) / 100) % 10 - 5
    }
    let id = 8772;

    let start = std::time::Instant::now();
    let mut grid = vec![0; 300*300];
    for y in 0..300 {
        for x in 0..300 {
            let c = x + y*300;
            grid[c] = powl(id, (x+1) as isize,(y+1)as isize);
        }
    }
    let mut ms = 0;
    let mut mc = (0,0,0);

    let mut sums = vec![0; 300*300];
    for sz in 0..300 {
        for y in 0..300-sz {
            let mut sum = 0;
            for x in 0..sz+1 {
                let cs = x+(y+sz)*300;
                sum += grid[cs];
            }
            sums[y*300] += sum;
            for x in 1..300-sz {
                sum -= grid[(x-1) + (y+sz)*300];
                sum += grid[(x+sz) + (y+sz)*300];
                sums[(x)+y*300] += sum;
            }
        }
        for x in 0..300-sz {
            if sz == 0 { break }
            let mut sum = 0;
            for y in 0..sz {
                let cs = (x+sz)+y*300;
                sum += grid[cs];
            }
            sums[x] += sum;
            for y in 1..300-sz {
                sum -= grid[(x+sz) + (y-1)*300];
                sum += grid[(x+sz) + (y+sz-1)*300];
                sums[x+y*300] += sum;
            }
        }
        for y in 0..300-sz {
            for x in 0..300-sz {
                let sum = sums[x+y*300];
                if sum > ms {
                    ms = sum;
                    mc = (x+1,y+1,sz+1);
                }
            }
        }
    }
    println!("mc: {:?}", mc);
    println!("elapsed: {:?}", start.elapsed());
}
