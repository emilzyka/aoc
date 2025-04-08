use std::{fs, time::Instant};


fn build() -> Vec<String> {
    let input = fs::read_to_string("input.txt")
    .expect("Failed to read input file");

    let map: Vec<String> = input
        .split_terminator('\n')
        .map(|l| {
            l.split_whitespace()
            .map(|e| e.parse::<String>().unwrap())
            .collect()
        })
        .collect();

    map
}

fn main() {
    let input = build();
    bench("part1", || calc_xmas(input.clone()));
    bench("part2", || calc_mas(input.clone()));
}

fn calc_xmas(input: Vec<String>) -> u32 {
    input.iter().enumerate().map(|(outer_index, line)|{
        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        chars.iter().enumerate().map(|(inner_index, c)| {
            let mut v = 0;
            if *c == 'X' {
                
                // region: -- horizontal 
               
                // horizontal right
                if inner_index + 3 < chars.len() 
                    && chars[inner_index + 1] == 'M'
                    && chars[inner_index + 2] == 'A'
                    && chars[inner_index + 3] == 'S' {
                        v += 1;
                }

                // horizontal right
                if inner_index >= 3 
                    && chars[inner_index - 1] == 'M'
                    && chars[inner_index - 2] == 'A'
                    && chars[inner_index - 3] == 'S' {
                        v += 1;
                }
                // endregion: -- horizontal 

                // region: -- vertical
                
                // vertical up
                if outer_index >= 3  
                    && input[outer_index - 1].chars().nth(inner_index) == Some('M')
                    && input[outer_index - 2].chars().nth(inner_index) == Some('A')
                    && input[outer_index - 3].chars().nth(inner_index) == Some('S') {
                        v += 1;   
                }

                // vertical down
                if outer_index + 3 < input.len() 
                    && input[outer_index + 1].chars().nth(inner_index) == Some('M')
                    && input[outer_index + 2].chars().nth(inner_index) == Some('A')
                    && input[outer_index + 3].chars().nth(inner_index) == Some('S') {
                       v += 1;   
               }


                // endregion: -- vertical

                // region: -- diagonal
                
                // diagonal right down
                if inner_index + 3 < chars.len() && outer_index + 3 < input.len()  
                    && input[outer_index + 1].chars().nth(inner_index + 1) == Some('M')
                    && input[outer_index + 2].chars().nth(inner_index + 2) == Some('A')
                    && input[outer_index + 3].chars().nth(inner_index + 3) == Some('S') {
                        v += 1;   
                }

                // diagonal right up
                if inner_index + 3 < chars.len() && outer_index >= 3  
                    && input[outer_index - 1].chars().nth(inner_index + 1) == Some('M')
                    && input[outer_index - 2].chars().nth(inner_index + 2) == Some('A')
                    && input[outer_index - 3].chars().nth(inner_index + 3) == Some('S') {
                        v += 1;   
                }
                
                // diagonal left up
                if inner_index >= 3 && outer_index + 3 < input.len()   
                && input[outer_index + 1].chars().nth(inner_index - 1) == Some('M')
                && input[outer_index + 2].chars().nth(inner_index - 2) == Some('A')
                && input[outer_index + 3].chars().nth(inner_index - 3) == Some('S') {
                    v += 1;   
                } 

                // diagonal left up
                   if inner_index >= 3 && outer_index >= 3  
                   && input[outer_index - 1].chars().nth(inner_index - 1) == Some('M')
                   && input[outer_index - 2].chars().nth(inner_index - 2) == Some('A')
                   && input[outer_index - 3].chars().nth(inner_index - 3) == Some('S') {
                       v += 1;   
                }

                // endregion: -- diagonal

            }
            v
        }).sum::<u32>()
    }).sum::<u32>()
}


fn calc_mas(input: Vec<String>) -> u32 {
    input.iter().enumerate().map(|(outer_index, line)|{
        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        chars.iter().enumerate().map(|(inner_index, c)| {
            let mut v = 0;
            if *c == 'A' 
                && outer_index > 0 && inner_index > 0
                && outer_index + 1 < input.len() && inner_index + 1 < chars.len() {
                if  (input[outer_index - 1].chars().nth(inner_index - 1) == Some('M') 
                    && input[outer_index + 1].chars().nth(inner_index + 1) == Some('S')
                    || input[outer_index - 1].chars().nth(inner_index - 1) == Some('S')
                    && input[outer_index + 1].chars().nth(inner_index + 1) == Some('M'))
                && 
                    (input[outer_index - 1].chars().nth(inner_index + 1) == Some('M')
                    && input[outer_index + 1].chars().nth(inner_index - 1) == Some('S')
                    || input[outer_index - 1].chars().nth(inner_index + 1) == Some('S') &&
                    input[outer_index + 1].chars().nth(inner_index - 1) == Some('M')) {
                    v += 1;
                }
            }
            v
        }).sum::<u32>()
    }).sum::<u32>()
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
        let input = build();
        let result = calc_xmas(input);
        assert_eq!(2575, result);

    }

    #[test]
    fn test_part2() {
    }
}