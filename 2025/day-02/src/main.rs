use std::{fs, time::Instant};
use rayon::prelude::*;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File expected");
    let ranges: Vec<&str> = content.lines()
        .map(|l| 
            l.split(",")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>())
        .flatten()
        .collect();
    
    bench("part_1 as parallel:", || sum_invalid_ids_parallel_part1(&ranges));
    bench("part_2 as parallel:", || sum_invalid_ids_parallel_part2(&ranges));

}

fn sum_invalid_ids_parallel_part1(ranges: &[&str]) -> i64 {
    ranges
        .par_iter()
        .map(|r| search_invalid_ids_parallel(r, check_reverse))
        .sum()
}

fn sum_invalid_ids_parallel_part2(ranges: &[&str]) -> i64 {
    ranges
        .par_iter()
        .map(|r| search_invalid_ids_parallel(r, check_repated))
        .sum()
}

fn search_invalid_ids_parallel<F>(range: &str, checker: F) -> i64 
    where 
        F: Fn(&str) -> bool, {

    let mut invalid_ids_sum = 0;

    let (start_string, end_sring) = range.split_once("-").unwrap();

    let start: i64 = start_string.parse().unwrap();
    let end: i64 = end_sring.parse().unwrap();
    
    for number in start..(end + 1) {
        if checker(&number.to_string()) {
            invalid_ids_sum += number;
        }
    }

    invalid_ids_sum
}    


fn check_reverse(number_string: &str) -> bool {
    let right = &number_string[0.. (number_string.len() / 2)];
    let left = &number_string[(number_string.len() / 2)..];

    match right == left {
        true => true,
        false => false
    }
} 

// for part 2
fn check_repated(number_string: &str) -> bool {
    let mut number_index = 0;

    while number_index < number_string.len() / 2 {
        if reapted_pattern(&number_string[0..number_index + 1], &number_string[number_index + 1..]) {
            return true
        }

        number_index += 1
    }

    false
} 

// for part 2
fn reapted_pattern(pattern_s: &str, number_s: &str) -> bool {
    let chunk_size = pattern_s.len(); 
    
    let c: Vec<&str> = number_s.as_bytes()
        .chunks(chunk_size)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect();
    
    if c.iter().all(|&v| v == pattern_s) {
        return true
    }
    
    false
}

fn bench<F: FnOnce() -> i64>(name: &str, f: F) {
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
        let content = fs::read_to_string("input.txt").expect("File expected");
        let ranges: Vec<&str> = content.lines()
        .map(|l| 
            l.split(",")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>())
        .flatten()
        .collect();
        let result = sum_invalid_ids_parallel_part1(&ranges);
        println!("test_part1 result: {}", result);
        assert_eq!(32976912643, result);
    }

    #[test]
    fn test_part2() {
        let content = fs::read_to_string("input.txt").expect("File expected");
        let ranges: Vec<&str> = content.lines()
        .map(|l| 
            l.split(",")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>())
        .flatten()
        .collect();
        let result = sum_invalid_ids_parallel_part2(&ranges);
        println!("test_part2 result: {}", result);
        assert_eq!(54446379122, result);
    }
}