use std::time::{Instant};
use std::fs::{File};
use std::io::{BufRead,BufReader};

/// return unique minimal value from an iterator by key function
fn unique_min_by<V, K, I, F>(iter: I, mut f: F) -> Option<V>
    where K: Ord + Eq,
          I: Iterator<Item=V>,
          F: FnMut(&V) -> K
{
    enum Min<K, V> {
        Empty,
        Uniq(K, V),
        Dup(K, V),
    }

    let mut min = Min::Empty;
    for item in iter {
        let key = f(&item);
        min = match min {
            Min::Empty => Min::Uniq(key, item),
            Min::Uniq(ref k, _) if *k > key => Min::Uniq(key, item),
            Min::Dup(ref k, _) if *k > key => Min::Uniq(key, item),
            Min::Uniq(ref k, _) if *k == key => Min::Dup(key, item),
            orig @ _ => orig,
        }
    }
    match min {
        Min::Empty | Min::Dup(_, _) => None,
        Min::Uniq(_, v) => Some(v)
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let fd = File::open("06.txt")?;
    let reader = BufReader::new(fd);
    let mut inp = Vec::<(isize,isize)>::new();
    for line in reader.lines().filter_map(Result::ok) {
        let inps = line.trim().split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<isize>>();
        inp.push((inps[0],inps[1]));
    }
    let start = Instant::now();

    let mut count = vec![0_usize; inp.len()];
    let mut infinite = vec![false; inp.len()];
    fn md(a: (isize,isize), b: (isize,isize)) -> isize {
        (a.0-b.0).abs() + (a.1-b.1).abs()
    }

    for y in 0..400 {
        for x in 0..400 {
            let closest = unique_min_by(inp.iter().enumerate(), |(_idx, &coord)| md(coord, (x,y)));
            if let Some((idx,_)) = closest {
                count[idx] += 1;
                if x == 0 || x == 399 || y == 0 || y == 399 {
                    infinite[idx] = true;
                }
            }
        }
    }

    let part1 = count.iter().enumerate()
        .filter(|(k,_)|!infinite[*k])
        .map(|(_,v)|v).max();

    let mut part2 = 0;
    for y in -500..1000 {
        for x in -500..1000 {
            let dist = inp.iter().map(|&coord| md(coord, (x,y))).sum::<isize>();
            if dist < 10000 {
                part2 += 1;
            }
        }
    }

    let elapsed = start.elapsed();

    println!("{:?}", part1);
    println!("{:?}", part2);
    println!("elapsed: {:?}", elapsed);
    Ok(())
}
