use std::{fs, time::Instant};
use rayon::prelude::*;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Filed expected");

    let ranges: Vec<(u64, u64)> = content
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line
                .split_once('-')
                .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                .unwrap()
    }).collect();


    let ingredient_ids: Vec<u64> = content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    bench("part_1", || count_fresh_ingredients(&ingredient_ids, &ranges));
    bench("part_2", || count_fresh_ids(&ranges));
}

fn count_fresh_ingredients(ids: &Vec<u64>, ranges: &Vec<(u64, u64)>) -> u64 {
    ids
        .par_iter()
        .map(|id| check_ingredient(id, &ranges))
        .sum()
}

fn check_ingredient(id: &u64, ranges: &Vec<(u64, u64)>) -> u64 {
    u64::from(ranges.iter().any(|(min, max)| id >= min && id <= max))
}


// part 2
fn count_fresh_ids(ranges: &Vec<(u64, u64)>) -> u64 {

    let mut valid_ranges: Vec<(u64, u64)> = ranges
        .iter()
        .map(|&r| r)
        .collect();

    valid_ranges.sort(); 

    let merged = merge_ranges(&valid_ranges);
    count_ranges(&merged)
}

fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {

    let mut merged: Vec<(u64, u64)> = vec![ranges[0]];
    
    for &(min, max) in &ranges[1..] {
        let last_idx = merged.len() - 1;
        if merged[last_idx].1 >= min {
            merged[last_idx].1 = merged[last_idx].1.max(max);
        } else {
            merged.push((min, max));
        }
    }
    merged
}

fn count_ranges(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|(min, max)| (max - min) + 1)
        .sum()
}

fn bench<F: FnOnce() -> u64>(name: &str, f: F) {
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{}: took {:?}. result: {}.", name, duration, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_input() -> (Vec<(u64, u64)>,  Vec<u64>) {
        let content = fs::read_to_string("input.txt").expect("Filed expected");
        let ranges: Vec<(u64, u64)> = content
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                line
                    .split_once('-')
                    .map(|(x, y)| (x.parse::<u64>().unwrap(), y.parse::<u64>().unwrap()))
                    .unwrap()
            }).collect();
        let ingredient_ids: Vec<u64> = content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
        (ranges, ingredient_ids)
    }


    #[test]
    fn test_part1() {
        let (ranges, ids) = load_input();
        let result = count_fresh_ingredients(&ids, &ranges);
        println!("test_part1 result: {}", result);
        assert_eq!(756, result);
    }

    #[test]
    fn test_part2() {
        let (ranges, _ids) = load_input();
        let result = count_fresh_ids(&ranges);
        println!("test_part2 result: {}", result);
        assert_eq!(355555479253787, result);
    }
}