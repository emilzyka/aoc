use std::{collections::HashMap, fs};
use std::time::Instant;

fn build() -> (Vec<u32>, Vec<u32>) {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");

    let numbers: Vec<u32> = input
    .split_whitespace()
    .map(|e| e.parse::<u32>().unwrap())
    .collect();

    let (mut l1, mut l2) = (Vec::new(), Vec::new());

    for (index, &num) in numbers.iter().enumerate() {
        if index % 2 == 0 {
            l1.push(num);
        } else {
            l2.push(num);
        }
    }

    (l1, l2)
}

fn main() {
    let (l1, l2) = build();
    bench("part_1", || calc_distance(&l1, &l2));
    bench("part_2", || calc_similarity(&l1, &l2));
}

fn calc_distance(l1: &[u32], l2: &[u32]) -> u32 {
    let mut l1 = l1.to_vec();
    let mut l2 = l2.to_vec();

    l1.sort_unstable();
    l2.sort_unstable();

    l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}
    
fn calc_similarity(l1: &[u32], l2: &[u32]) -> u32 {

    let mut lookup: HashMap<u32, u32> = HashMap::new();

    l1
        .iter()
        .map(|&e|{
            let v = lookup.entry(e).or_insert_with(|| {
                let o = l2.iter().filter(|&&x| x == e).count() as u32;
                o * e
            });

            *v

        })
        .sum()
}

fn bench<F: FnOnce() -> u32>(name: &str, f: F) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{}: took {:?}. result: {}.", name, duration, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (l1, l2) = build();
        let result: u32 = calc_distance(&l1, &l2);
        println!(">> {}", result);
        assert_eq!(2196996, result);
    }

    #[test]
    fn test_part2() {
        let (l1, l2) = build();
        let result = calc_similarity(&l1, &l2);
        println!(">> {}", result);
        assert_eq!(23655822, result);
    }
}