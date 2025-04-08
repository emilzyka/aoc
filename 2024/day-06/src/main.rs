use std::{collections::HashSet, fs, time::Instant};

fn build() -> Vec<Vec<char>> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    input
        .lines()
        .map(|l| {
            l
                .chars()
                .collect() 
        }).collect()
}

fn main() {
    let map = build();
    bench("part1", || track_path_count_visited(map.clone()));
    bench("part2", || count_pos_for_new_obstacle(map.clone()));
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn check_if_obstacle(char: char) -> bool {
    match char {
        '#' => true,
        _ => false,
    }
}

fn get_start_pos(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^'){
            return Some((j, i));
        }
    }
    None
}

fn is_in_bounds(pos: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    pos.0 < map[0].len() && pos.1 < map.len()
}

fn track_path_count_visited(map: Vec<Vec<char>>) -> u32 {
    
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    
    // -- Movement --
    let mut pos = get_start_pos(&map).unwrap();
    let mut dir = Direction::Up;
    let mut in_bounds = true;

    // -- Apply movements --
    while in_bounds {
        let (x, y) = pos; 

        if !visited.contains(&pos) {
            visited.insert(pos);
        }
        
        // -- Get next position --
        let next_pos = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if is_in_bounds(next_pos, &map) {
            match check_if_obstacle(map[next_pos.1][next_pos.0]) {
                true => dir = get_direction(dir),
                false => pos = next_pos,
            }
        } else {
            in_bounds = false;
        }
    }

    visited.len() as u32
}

// -- Modified for part 2 retuning path instead 
fn track_path_return_visited(map: Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    
    // -- Movement 
    let mut pos = get_start_pos(&map).unwrap();
    let mut dir = Direction::Up;
    let mut in_bounds = true;

    // -- Apply movements 
    while in_bounds {
        let (x, y) = pos; 

        if !visited.contains(&pos) {
            visited.insert(pos);
        }
        
        // -- Get next position 
        let next_pos = match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        if is_in_bounds(next_pos, &map) {
            match check_if_obstacle(map[next_pos.1][next_pos.0]) {
                true => dir = get_direction(dir),
                false => pos = next_pos,
            }
        } else {
            in_bounds = false;
        }
    }

    visited
}

fn is_stuck_on_current_map(map: Vec<Vec<char>>, obs_pos: (usize, usize)) -> bool {
    
    let mut fst_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut snd_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut times_passed_obstacle = 0;
    let mut c_move = 0;

    // -- Movement 
    let mut pos = get_start_pos(&map).unwrap();
    let mut dir = Direction::Up;
    let mut in_bounds = true;

    // -- Apply movements
    while in_bounds {
        let (x, y) = pos; 

        if times_passed_obstacle % 2 != 0 && times_passed_obstacle > 0 {
            if !fst_visited.contains(&pos) {
                fst_visited.insert(pos);
            }
        }

        if times_passed_obstacle % 2 == 0 && times_passed_obstacle > 0 {
            if !snd_visited.contains(&pos) {
                snd_visited.insert(pos);
            }
        }
     
        // -- Get next position 
        let next_pos = match dir {
            Direction::Up => {
                if y == 0 {
                    break;
                }

                (x, y - 1)
            },
            Direction::Down => (x, y + 1),
            Direction::Left => {
                if x == 0 {
                    break;
                }

                (x - 1, y)
            },
            Direction::Right => (x + 1, y),
        };

        if next_pos == obs_pos {
            if times_passed_obstacle > 0 && fst_visited == snd_visited {
                return true;
            } else if times_passed_obstacle % 2 == 0 {
                fst_visited.clear();
                snd_visited.clear();
            }
            times_passed_obstacle += 1;
        }

        if is_in_bounds(next_pos, &map) {
            match check_if_obstacle(map[next_pos.1][next_pos.0]) {
                true => dir = get_direction(dir),
                false => pos = next_pos,
            }
        } else  {
            in_bounds = false;
        }

        if c_move > 7500 {
            return true;
        }

        c_move += 1;
    }
    false
}


fn count_pos_for_new_obstacle(map: Vec<Vec<char>>) -> u32 {
    
    // -- Trackers 
    let mut visited = track_path_return_visited(map.clone());
    let mut acc = 0;
    let start_pos = get_start_pos(&map).unwrap();

    visited.remove(&start_pos);

    for pos in visited {
        // -- Generate new map with obstacle 
        let mut new_map = map.clone();
        new_map[pos.1][pos.0] = '#';

        // -- Check if we are stuck on the new map 
        let obs_pos = (pos.0, pos.1);
        if is_stuck_on_current_map(new_map, obs_pos) {
            acc += 1;
        } 
    }

    acc
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
        let map = build();
        let result = track_path_count_visited(map);
        assert_eq!(result, 4977);
    }

    #[test]
    fn test_part2() {
        let map = build();
        let result = count_pos_for_new_obstacle(map);
        assert_eq!(result, 1729);
    }
}