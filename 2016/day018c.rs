use std::collections::HashMap;

const START: u64 = 0b0110010001001101110111011110001111010101110110110;
fn main() {
    let mut row = START;
    let mask: u64 = (1<<49) - 1;
    let mut visited: HashMap<u64, u64> = HashMap::new();
    let mut safe = row.count_zeros();
    for i in 0..u64::max_value() {
        println!("{:049b}", row);
        if visited.contains_key(&row) {
            println!("loop! {}", i);
            break;
        }
        visited.insert(row, i);
        row = ((row << 1) ^ (row >> 1)) & mask;
        safe += row.count_zeros();
    }
}
