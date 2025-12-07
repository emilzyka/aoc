use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("File expected");

    let numbers: Vec<Vec<i64>> = content
        .lines()
        .take_while(|l| !l.starts_with(&['*', '+', '-']))
        .map(|l| {
            l
                .split_ascii_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect()
            })
        .collect();

    let operations: Vec<char> = content
            .lines()
            .skip_while(|l| !l.starts_with(&['*', '+', '-']))
            .flat_map(|l| {
                l
                    .split_ascii_whitespace()
                    .map(|op| op.parse::<char>().unwrap())
                })
            .collect();


    println!("result: {}", solve_math(&numbers, &operations));

    println!("result: {}", part2());

}


fn solve_math(numbers: &Vec<Vec<i64>>, operations: &Vec<char>) -> i64 {
    let ordered = order_numbers(numbers);
    apply_operations(ordered, operations, numbers.len())
}


fn apply_operations(numbers: Vec<i64>, operations: &Vec<char>, step_by: usize) -> i64 {
    let mut count = 0;

    let mut operation_index = 0;
    let max = numbers.len().saturating_sub(step_by + 1);

    for i in (0..=max+1).step_by(step_by) {
        let mut nums: Vec<i64> = vec![];
        for z in 0..step_by {
            nums.push(numbers[i + z])
        }

        let op = operations[operation_index];

        match op {
            '+' => count += nums.iter().sum::<i64>(),
            '*' => count += nums.iter().product::<i64>(),
            _ => {}
        }
        operation_index += 1;
    }
    count
}

fn order_numbers(numbers: &Vec<Vec<i64>>) -> Vec<i64> {

    let mut count_array: Vec<i64> = vec![];

    for index in 0..numbers[0].len() {
        for nums in numbers {
            count_array.push(nums[index]);
        }
    }
 
    count_array
}

// part 2
fn part2() -> u64 {
    let content = fs::read_to_string("input.txt").expect("File expected");


    let operations: String = content
        .lines()
        .skip_while(|l| !l.starts_with(&['*', '+', '-']))
        .collect();

    let numbers: Vec<Vec<char>> = content
        .lines()
        .take_while(|l| !l.starts_with(&['*', '+', '-']))
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut count: u64 = 0;
    let mut calcs: Vec<u64> = vec![];

    let mut char_index = 0;
    let mut last_i = 0;
    for (i, c) in operations.chars().enumerate().skip(1) {
        if !c.is_whitespace() || i == operations.len() - 1 {
            for number_index in (last_i..i).rev() {
                // 4..0
                let mut lr_numbers: Vec<u64> = vec![];
                
                for cols in 0..numbers.len() {

                    if let Some(char) = &numbers[cols][number_index].to_digit(10) {
                        lr_numbers.push(*char as u64);
                    }
                }

                if lr_numbers.len() > 0 {
                    calcs.push(lr_numbers.iter().fold(0, |acc, d| acc * 10 + d));
                }
            }
            
            let op = &operations.chars().nth(last_i).unwrap();
            
            match op {
                '+' => count += calcs.iter().sum::<u64>(),
                '*' => count += calcs.iter().product::<u64>(),
                _ => {}
            }

            calcs.clear();
            last_i = i;
            char_index = i;
        } else {
            char_index += 1;
        }
    }

    // len of operations total count of ops performed.
    // 0 means magic number, do nothing
    count
  
}