use std::time::Instant;
use rustc_hash::FxHashMap;
fn simulate_stones(initial_stones: Vec<i64>, iterations: usize) -> usize {
    let mut stones: FxHashMap<i64, usize> = FxHashMap::default();
    for stone in initial_stones {
        *stones.entry(stone).or_insert(0) += 1;
    }
    let start = Instant::now(); // Start timer
    for i in 0..iterations {
        let mut new_stones: FxHashMap<i64, usize> = FxHashMap::default();
        for (stone, count) in stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else {
                let stone_str = stone.to_string();
                let len = stone_str.len();
                if len % 2 == 0 {
                    let mid = len / 2;
                    let left: i64 = stone_str[..mid].parse().unwrap();
                    let right: i64 = stone_str[mid..].parse().unwrap();
                    *new_stones.entry(left).or_insert(0) += count;
                    *new_stones.entry(right).or_insert(0) += count;
                } else {
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            }
        }
        stones = new_stones;
        println!("Iteration {} completed in {:?}", i + 1, start.elapsed());
    }
    stones.values().sum()
}
fn main() {
    let initial_stones = vec![6571, 0, 5851763, 526746, 23, 69822, 9, 989];
    let iterations = 25;
    let total_stones = simulate_stones(initial_stones, iterations);
    println!("Total stones after {} iterations: {}", iterations, total_stones);
}