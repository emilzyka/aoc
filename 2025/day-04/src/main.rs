use std::{fs, time::Instant};

/*
result: 1560
result: 9609
*/

fn main() {
    let content = fs::read_to_string("input.txt").expect("File expected");

    let rolls_of_paper: Vec<Vec<u8>>= content
        .lines()
        .map(|l| {
            l.chars().map(|c| {
                if c == '@' {
                    1 as u8
                } else {
                    0 as u8
                }
            }).collect::<Vec<u8>>()
        }).collect();

    bench("part_1",|| forklift_accessible_papers(&rolls_of_paper));
    bench("part_2",|| forklift_accessible_papers_rec(&rolls_of_paper, 0));
}

// part 1
fn forklift_accessible_papers(map: &Vec<Vec<u8>>) -> i32 {
    
    let mut accessible_papers = 0;

    // pos is x (inner_vec pos), y (outer_vec pos)
    for (y, map_x) in map.iter().enumerate() {
        for (x, paper) in map_x.iter().enumerate() {
            
            let mut paper_count = 5;

            if *paper == 1 {
                let postion_to_check = get_valid_positions((x, y), map);
                paper_count = postion_to_check.find_adjacent_papers((x, y), map);
            }
            
            if paper_count < 4 {
                accessible_papers += 1;
            }
        }
    }
    
    accessible_papers
}

// part 2 
fn forklift_accessible_papers_rec(map: &Vec<Vec<u8>>, acc: i32) -> i32 {
    
    let mut accessible_papers = 0;
    let mut new_map = map.clone();

    // pos is x (inner_vec pos), y (outer_vec pos)
    for (y, map_x) in map.iter().enumerate() {
        for (x, paper) in map_x.iter().enumerate() {
            
            let mut paper_count = 5;

            if *paper == 1 {
                let postion_to_check = get_valid_positions((x, y), &map);
                paper_count = postion_to_check.find_adjacent_papers((x, y), &map);
            }
            
            if paper_count < 4 {
                accessible_papers += 1;
                new_map[y][x] = 0;
            }
        }
    }
    
    // base case
    if accessible_papers == 0 {
        return acc;
    }

    return forklift_accessible_papers_rec(&new_map, acc + accessible_papers);
}


struct PositionsToCheck {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    d_up_left: bool,
    d_down_left: bool,
    d_up_right: bool,
    d_down_right: bool,
}

impl PositionsToCheck {
    fn new(up: bool, down: bool, left: bool, right: bool, 
        d_up_left: bool, d_down_left: bool, d_up_right: bool, d_down_right: bool) -> Self {
        PositionsToCheck { up, down, left, right, d_up_left, d_down_left, d_up_right, d_down_right }
    }

    fn find_adjacent_papers(&self, (x, y): (usize, usize), map: &Vec<Vec<u8>>) -> u8 {
        let directions = [
            (self.up, (0, -1)),
            (self.down, (0, 1)),
            (self.left, (-1, 0)),
            (self.right, (1, 0)),
            (self.d_up_left, (-1, -1)),
            (self.d_down_left, (-1, 1)),
            (self.d_up_right, (1, -1)),
            (self.d_down_right, (1, 1))
        ];

        directions.iter()
            .filter(|(is_valid, _)| *is_valid)
            .filter(|(_, (dir_x, dir_y))| {
                let new_y = y as isize + dir_y;
                let new_x = x as isize + dir_x;
                map[new_y as usize][new_x as usize] == 1
            })
            .count() as u8
    }
}


fn get_valid_positions((x, y): (usize, usize), map: &Vec<Vec<u8>>) -> PositionsToCheck {

    // pos is x (inner_vec pos), y (outer_vec pos)'

    let height = map.len();
    let width = map[0].len();

    let up = y > 0;
    let down = y + 1 < height;
    let left = x > 0;
    let right = x + 1 < width;
    
    // diagonals

    let d_up_left = up && left;
    let d_down_left = down && left;
    let d_up_right = up && right;
    let d_down_right = down && right;

    PositionsToCheck::new(up, down, left, right, d_up_left, d_down_left, d_up_right, d_down_right)
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
        
        let content = fs::read_to_string("input.txt").expect("File expected");
    
        let rolls_of_paper: Vec<Vec<u8>>= content
        .lines()
        .map(|l| {
            l.chars().map(|c| {
                if c == '@' {
                    1 as u8
                } else {
                    0 as u8
                }
            }).collect::<Vec<u8>>()
        }).collect();
        
        let result = forklift_accessible_papers(&rolls_of_paper);
        println!("test_part1 result: {}", result);
        assert_eq!(1560, result);
    }

    #[test]
    fn test_part2() {
        
        let content = fs::read_to_string("input.txt").expect("File expected");
    
        let rolls_of_paper: Vec<Vec<u8>>= content
        .lines()
        .map(|l| {
            l.chars().map(|c| {
                if c == '@' {
                    1 as u8
                } else {
                    0 as u8
                }
            }).collect::<Vec<u8>>()
        }).collect();
        
        let result = forklift_accessible_papers_rec(&rolls_of_paper, 0);
        println!("test_part2 result: {}", result);
        assert_eq!(9609, result);
    }
}