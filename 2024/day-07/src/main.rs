use std::{fs, io::Cursor, thread::Builder, time::Instant};

/* 
190: 10 19 
3267: 81 40 27
(u32, Vec<u32>)
*/

fn build() ->  Vec<(u32, Vec<u32>)> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    input
        .lines()
        .map(|l| {
            let mut parts = l.split(":");
            
            let key = parts.next().unwrap().parse::<u32>().unwrap();    
            
            let values = parts.next().unwrap().split_whitespace()
                .map(|s| s.parse::<u32>().expect("Failed to parse value")).collect::<Vec<u32>>();

            (key, values)
        })
        .collect()
}

fn main() {
    let input = build();
    test(input.clone());
}

fn test(input: Vec<(u32, Vec<u32>)>) -> () {
    input
        .iter()
        .for_each(|(key, value)| {
            let res = is_possible_true((*key, value.clone()));
            println!("{}", res);
        })
}

fn is_possible_true(input: (u32, Vec<u32>)) -> bool {
   
   // store operation results in a hasmap
    let mut results = std::collections::HashSet::new();

    for i in 1..input.1.len(){
        let prev = input.1[i-1];
        let curr = input.1[i];

        results.insert(prev + prev);
        results.insert(prev * curr);
    }

    if results.contains(&input.0) {
        return true;
    }

    false
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
     
    }

    #[test]
    fn test_part2() {

    }
}