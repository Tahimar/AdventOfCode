use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn sort_list(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

pub fn calculate_distance(num1: i32, num2: i32) -> i32 {
    (num1 - num2).abs()
}

pub fn calculate_similarity(num1: i32, amount: i32) -> i32 {
    (num1 * amount).abs()
}

pub fn read_numbers_from_file(filepath: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let file = File::open(Path::new(filepath))?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    Ok((list1, list2))
}

pub fn calculate_total_distance() -> i32 {
    let (list1, list2) = read_numbers_from_file("Inputs/inputDay1.txt").unwrap();
    let sorted1 = sort_list(list1);
    let sorted2 = sort_list(list2);

    sorted1.iter()
        .zip(sorted2.iter())
        .map(|(a, b)| calculate_distance(*a, *b))
        .sum()
    
}

pub fn calculate_total_similarity() -> i32 {
    let (list1, list2) = read_numbers_from_file("Inputs/inputDay1.txt").unwrap();

    list1.iter()
        .map(|&num| {
            let count = list2.iter().filter(|&&x| x == num).count() as i32;
            calculate_similarity(num, count)
        })
        .sum()
}