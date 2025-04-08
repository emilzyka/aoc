use std::{collections::{hash_map, HashMap}, fs, time::Instant};

fn build_instructions() -> Vec<Vec<u32>> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

        input
            .lines()
            .skip_while(|&line| !line.trim().is_empty()) 
            .skip(1) 
            .filter(|line| !line.trim().is_empty()) 
            .map(|line| {
                line.split(',')
                    .filter_map(|s| s.trim().parse::<u32>().ok()) 
                    .collect::<Vec<u32>>() 
            })
            .collect()
    
}

fn build_rules() -> Vec<(u32, u32)> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
    
    input
        .lines()
        .take_while(|&l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split('|');
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        
        }).collect()    
}

fn main() {
    let rules = build_rules();
    let instructions = build_instructions();

    bench("part1", || calc_mid_page(rules.clone(), instructions.clone()));
    bench("part2", || calc_mid_page_invalids(rules.clone() , instructions.clone()));

}


fn calc_mid_page(rules: Vec<(u32, u32)>, instructions: Vec<Vec<u32>>) -> u32 {
    
    let mut valid_instructions: Vec<Vec<u32>>  = vec![];
    let rule_map: HashMap<u32, Vec<u32>> = create_ruleset_lookup(rules);

    for i in instructions {
        let mut is_valid = true;
        for index in 1..i.len() {
            let prev = i[index - 1]; 
            let curr = i[index];

            match rule_map.get(&curr) {
                Some(v) => {
                    if v.contains(&prev) {
                        is_valid = false;
                        break;
                    }
                } 
                None => ()
            }
        }

        if is_valid {
            valid_instructions.push(i);
        }
    }

    valid_instructions
        .iter()
        .filter_map(|v| {
            let mid = v.len() / 2;
            Some(v[mid])
        })
        .sum()
}



fn calc_mid_page_invalids(rules: Vec<(u32, u32)>, instructions: Vec<Vec<u32>>) -> u32 {
    
    let mut valid_instructions: Vec<Vec<u32>>  = vec![];
    let rule_map: HashMap<u32, Vec<u32>> = create_ruleset_lookup(rules);
    let mut valid_instruction_map: HashMap<Vec<u32>, u32> = hash_map::HashMap::new();

    // get the invalid instructions
    for i in instructions {
        let mut is_valid = true;
        for index in 1..i.len() {
            let prev = i[index - 1]; 
            let curr = i[index];

            match rule_map.get(&curr) {
                Some(v) => {
                    if v.contains(&prev) {
                        is_valid = false;
                        break;
                    }
                } 
                None => ()
            }
        }

        if !is_valid {
            let valid = invalid_made_valid(&rule_map, i.clone(), &valid_instruction_map);
            valid_instructions.push(valid.clone());
            valid_instruction_map.insert(valid.clone(), 0);

        } else {
            valid_instruction_map.insert(i.clone(),0);
        }
    }

    valid_instructions
        .iter()
        .filter_map(|v| {
            let mid = v.len() / 2;
            Some(v[mid])
        })
        .sum()
    
    
}


fn invalid_made_valid(
    rule_map: &HashMap<u32, Vec<u32>>, 
    mut invalid: Vec<u32>, 
    valid_instruction_map: &HashMap<Vec<u32>, u32>
) -> Vec<u32> {
        let mut is_restart = true;

        while is_restart {
            is_restart = false;
            for index in 1..invalid.len() {
                let prev = invalid[index - 1];
                let curr = invalid[index];  
                match rule_map.get(&curr) {
                    Some(v) => {
                        if v.contains(&prev) {
                            invalid[index] = prev;
                            invalid[index - 1] = curr;

                            if valid_instruction_map.contains_key(&invalid) {
                                return invalid;
                            } 

                            is_restart = true;
                            break;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }
    invalid
}

// part2: took 23.0364ms. result: 6191 before optimization


fn create_ruleset_lookup(rules: Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut ruleset = HashMap::new();

    for (key, value) in rules {
        ruleset.entry(key).or_insert_with(Vec::new).push(value);
    }

    ruleset
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
        let rules = build_rules();
        let instructions = build_instructions();
        let result = calc_mid_page(rules, instructions);
        assert_eq!(5275, result);
    }

    #[test]
    fn test_part2() {
        let rules = build_rules();
        let instructions = build_instructions();
        let result = calc_mid_page_invalids(rules, instructions);
        assert_eq!(6191, result);
    }
}