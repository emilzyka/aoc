use std::{fs, time::Instant};

// 0 - 99
// L0 -> 99, R99 -> 0
// Starts at 50
// Actual password, is totalt 0 after any roation

// L999, 

fn main() {
    let content = fs::read_to_string("input.txt").expect("File expecteds");
    let instructions = content.lines().collect::<Vec<&str>>();

    bench("part_1", || crack_lock(instructions));

    
}

fn crack_lock(instructions: Vec<&str>) -> i32 {

    let mut lock_value = 50;
    let mut total_zero = 0;

    let rotations: Vec<TurnDistance> = instructions.iter().map(|s| {
        if s.starts_with("L") {
            TurnDistance::Left(s[1..].parse().unwrap()) 
        } else {
            TurnDistance::Right(s[1..].parse().unwrap()) 
        }
    }).collect();

    for rot in rotations {

        let new_lock_value = rotate_dial(rot, lock_value);

        if new_lock_value == 0 {
            total_zero += 1;
        }

        lock_value = new_lock_value;
    }

    total_zero
}

pub enum TurnDistance {
    Left(i32),
    Right(i32),
}

fn rotate_dial(turn: TurnDistance, current: i32) -> i32 {
    let mut new = current;
    match turn {
        TurnDistance::Left(distance) => {
            let temp = new - distance; 
            if temp < 0 {
                return rotate_dial(TurnDistance::Left(temp.abs()), 100);
            } else {
                new = temp; 
            }
        }
        TurnDistance::Right(distance) => {
            let temp: i32 = new + distance;
            if temp > 100 {
                return rotate_dial(TurnDistance::Right(temp - 100), 0);
            } else {
                if temp == 100 {
                    new = 0;
                } else {
                    new = temp; 
                }
            }
        },
    }
    new
}

fn bench<F: FnOnce() -> i32>(name: &str, f: F) {
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

// 31, 37
// 99 - 37