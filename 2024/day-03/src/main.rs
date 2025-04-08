use std::fs;
use std::time::Instant;

use regex::Regex;


fn main() {
    bench("part1", || mul_corr_memo());
    bench("part2", || mul_corr_memo_do_dont());
}

fn mul_corr_memo() -> u32 {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap(); 

    let matches: Vec<&str> = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect();

    let matches_str = matches.join(",");

    let re_ope = Regex::new(r"\d{1,3},\d{1,3}").unwrap();

    let matche_ope: Vec<(u32, u32)> = re_ope
        .find_iter(&matches_str)
        .map(|m| {
            let p: Vec<u32> = m.as_str()
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            (p[0], p[1])
        } )
        .collect();
       
    
    let result: u32 = matche_ope
        .iter()
        .map(|e| e.0 * e.1)
        .sum();
    result
}

fn mul_corr_memo_do_dont() -> u32 {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");

    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do(n't)?\(\))").unwrap(); 

    let matches: Vec<&str> = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect();

    let mut valid_instruction = vec![];
    let mut is_do = true;

    for m in matches {
        match m {
            "don't()" => is_do = false,
            "do()" => is_do = true,
            _ => (),
        }

        if is_do {
            valid_instruction.push(m);
        }
    }

    let matches_str = valid_instruction.join(",");
    let re_ope = Regex::new(r"\d{1,3},\d{1,3}").unwrap();

    let matche_ope: Vec<(u32, u32)> = re_ope
        .find_iter(&matches_str)
        .map(|m| {
            let p: Vec<u32> = m.as_str()
                .split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            (p[0], p[1])
        } )
        .collect();
       
    
    let result: u32 = matche_ope
        .iter()
        .map(|e| e.0 * e.1)
        .sum();
    result

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
        let result = mul_corr_memo();
        assert_eq!(178886550, result);
    }

    #[test]
    fn test_part2() {
        let result = mul_corr_memo_do_dont();
        assert_eq!(87163705, result);
    }
}