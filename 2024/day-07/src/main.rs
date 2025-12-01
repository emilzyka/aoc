use std::fs;


fn build() -> Vec<(u32, Vec<char>)> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    input
        .lines()
        .map(|l| {
            l
                .split(":")
                .collect()
        }).collect()
}

fn main() {
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