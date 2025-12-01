use std::{fs, time::Instant};

fn main() {
    let content = fs::read_to_string("input.txt").expect("File expecteds");
    let instructions = content.lines().collect::<Vec<&str>>();

    bench("part_1", || crack_lock(instructions.clone()));
    bench("part_2", || crack_lock_method_0x434C49434B(instructions));
}

pub enum TurnDistance {
    Left(i32),
    Right(i32),
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


fn crack_lock_method_0x434C49434B(instructions: Vec<&str>) -> i32 {

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

        let new_lock_value = rotate_dial_acc(rot, lock_value, 0);

        if new_lock_value.0 == 0 {
            total_zero += 1;
        }

        if new_lock_value.1 > 0 {
            total_zero += new_lock_value.1;
        }

        lock_value = new_lock_value.0;
    }

    total_zero
}

fn rotate_dial_acc(turn: TurnDistance, current: i32, acc: i32) -> (i32, i32) {
    let mut new = current;
    match turn {
        TurnDistance::Left(distance) => {
            let temp = new - distance;
            if temp < 0 {
                match current {
                    0 => return rotate_dial_acc(TurnDistance::Left(temp.abs()), 100, acc),
                    _ => return rotate_dial_acc(TurnDistance::Left(temp.abs()), 100, acc + 1),
                }
            } else {
                new = temp; 
            }
        }
        TurnDistance::Right(distance) => {
            let temp: i32 = new + distance;
            if temp > 100 {
                return rotate_dial_acc(TurnDistance::Right(temp - 100), 0, acc + 1);
            } else {
                if temp == 100 {
                    new = 0;
                } else {
                    new = temp; 
                }
            }
        },
    }
    (new, acc)
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
        let content = fs::read_to_string("input.txt").expect("File expecteds");
        let instructions = content.lines().collect::<Vec<&str>>();
        let result = crack_lock(instructions);
        println!("test_part1 result: {}", result);
        assert_eq!(1026, result);
    }

    #[test]
    fn test_part2() {
        let content = fs::read_to_string("input.txt").expect("File expecteds");
        let instructions = content.lines().collect::<Vec<&str>>();
        let result = crack_lock_method_0x434C49434B(instructions);
        println!("test_part2 result: {}", result);
        assert_eq!(5923, result);
    }
}