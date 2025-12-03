use std::{fs, time::Instant};

use rayon::prelude::*;

fn main() {

    let content = fs::read_to_string("input.txt").expect("File expected");
    
    let battery_banks: Vec<&str> = content
        .lines()
        .collect();

    bench("total joltage: {}", || total_joltage_output(&battery_banks, 2));
    bench("total joltage: {}", || total_joltage_output2(&battery_banks, 12));

}

// part 1

fn total_joltage_output(battery_banks: &[&str], n: usize) -> u64 {
    battery_banks
        .par_iter()
        .map(|bank| get_all_n_combinations(&bank, n))
        .sum()
} 


fn get_all_n_combinations(number_s: &str, _n: usize) -> u64 {
 
    let mut  combinations: Vec<(u64, u64)> = vec![];

    for (index, value) in number_s.chars().enumerate() {
        for (_inner_index, inner_value) in number_s[1 + index..].chars().enumerate() {
            let outer_n = value.to_digit(10).unwrap() as u64;
            let inner_n =  inner_value.to_digit(10).unwrap() as u64;
            combinations.push((outer_n, inner_n));
        }
    }

   combinations.iter().max().map(|p| format!("{}{}",p.0, p.1).parse::<u64>().unwrap()).unwrap()
}

// part 2
fn total_joltage_output2(battery_banks: &[&str], n: usize) -> u64 {
    battery_banks
        .par_iter()
        .map(|bank| get_all_combinations_n(&bank, n))
        .sum()
} 

fn get_all_combinations_n(number: &str, size: usize) -> u64 {
    let mut values: Vec<u8> = vec![];
    let mut remove = number.len() - size;

    for c in number.bytes() {
        let d = c - b'0';

        while let Some(&last) = values.last() {
            if remove > 0 && d > last {
                values.pop();
                remove -= 1;
            } else {
                break;
            }
        }

        values.push(d);
    }

    values.truncate(size);

   as_number(&values)
}

fn as_number(d: &Vec<u8>) -> u64 {
    format!("{}{}{}{}{}{}{}{}{}{}{}{}",d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], d[8], d[9], d[10], d[11]).parse::<u64>().unwrap()
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

    #[test]
    fn test_part1() {
        
        let content = fs::read_to_string("input.txt").expect("File expected");
    
        let battery_banks: Vec<&str> = content
            .lines()
            .collect();
        
        let result = total_joltage_output(&battery_banks, 2);
        println!("test_part1 result: {}", result);
        assert_eq!(17554, result);
    }

    #[test]
    fn test_part2() {
        
        let content = fs::read_to_string("input.txt").expect("File expected");
    
        let battery_banks: Vec<&str> = content
            .lines()
            .collect();

        let result = total_joltage_output2(&battery_banks, 12);
        println!("test_part2 result: {}", result);
        assert_eq!(175053592950232, result);
    }
}