use std::fs;
use std::time::Instant;

fn build() -> Vec<Vec<u32>> {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");

    let numbers: Vec<Vec<u32>> = input
        .split_terminator('\n')
        .map(|l| {
            l.split_whitespace()
            .map(|e| e.parse::<u32>().unwrap())
            .collect()
        })
        .collect();

    numbers
}

fn main() {
    let input = build();
    bench("part_1", || number_safe_reports(input.clone()));
    bench("part_2", || number_safe_reports_damp(input));
}


fn number_safe_reports(reports: Vec<Vec<u32>>) -> u32 {
   
   let mut count = 0;

    for r in reports {
        if is_safe_report(&r) {
            count+=1;
        }
    }

   count
}

fn is_safe_report(v: &[u32]) -> bool {
    let is_inc = v[0] <= v[1]; 
    v.windows(2)
        .all(|p| {
            let (a, b) = (p[0], p[1]);
            let diff = a.abs_diff(b);
            (if is_inc { a <= b } else { a >= b}) && (diff >=1 && diff <= 3)
        })
}


fn number_safe_reports_damp(reports: Vec<Vec<u32>>) -> u32 {
    
    let mut count: u32 = 0;

    for r in reports {
        
        if is_safe_report(&r) {
            count += 1;
            continue;
        }
        
        for i in 0..r.len() {
            let mut sub = r.clone();
            sub.remove(i);

            if is_safe_report(&sub) {
                count += 1;
                break;
            }
        }               
    }
    count        
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
        let reports = build();
        let result = number_safe_reports(reports);
        println!(">> {}", result);
        assert_eq!(341, result);
    }

    #[test]
    fn test_part2() {
        let reports = build();
        let result = number_safe_reports(reports);
        println!(">> {}", result);
        assert_eq!(341, result);
    }
}