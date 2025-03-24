use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");
//(Vec<u32>, Vec<u32>)

    let numbers: Vec<u32> = input
    .split_whitespace()
    .map(|e| e.parse::<u32>().unwrap())
    .collect();

    let (mut l1, mut l2) = (Vec::new(), Vec::new());

    for (index, &num) in numbers.iter().enumerate() {
        if index % 2 == 0 {
            l1.push(num);
        } else {
            l2.push(num);
        }
    }

    // let distance = calculate_distances(l1, l2);
    // println!("{}", distance);

    let sim = calculate_similarity(l1, l2);
    println!("{}", sim);

}

fn calculate_distances(mut l1: Vec<u32>, mut l2: Vec<u32>) -> u32 {

    l1.sort_unstable();
    l2.sort_unstable();

    l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| distance(a, b))
        .sum()
}

fn distance(a: &u32, b: &u32) -> u32 {
    a.abs_diff(*b)
}
    
fn calculate_similarity(l1: Vec<u32>, l2: Vec<u32>) -> u32 {
    l1
        .iter()
        .map(|e|{
           let o = l2.iter().filter(|&&x| x == *e).count() as u32;
           o * e
        })
        .sum()
}