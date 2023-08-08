use itertools::Itertools;
use std::fs::read_to_string;

//TODO: Refactor common parts of pairs_partially_contained and pairs_fully_contained
fn main() {
    let input_string = input_to_string("./input.txt").unwrap();
    println!(
        "the first answer is: {:#?}",
        pairs_fully_contained(input_string.clone())
    );
    println!(
        "the second answer is: {:#?}",
        pairs_partially_contained(input_string)
    );
}

fn input_to_string(file_path: &str) -> Result<String, &str> {
    match read_to_string(file_path) {
        Ok(input) => Ok(input),
        Err(_) => Err("Please put input.txt in executable directory"),
    }
}

fn pairs_fully_contained(input: String) -> i32 {
    let mut total_pairs_contained = 0;

    for line in input.lines() {
        let (first_pair, second_pair): (Vec<i32>, Vec<i32>) = line
            .split(',')
            .map(|s| {
                let mut s = str_to_int_range(s);
                s.sort();
                s
            })
            .collect_tuple()
            .unwrap();

        if first_pair.iter().all(|x| second_pair.contains(x))
            || second_pair.iter().all(|x| first_pair.contains(x))
        {
            total_pairs_contained += 1;
        }
    }

    total_pairs_contained
}

fn pairs_partially_contained(input: String) -> i32 {
    let mut total_pairs_contained = 0;

    for line in input.lines() {
        let (first_pair, second_pair): (Vec<i32>, Vec<i32>) = line
            .split(',')
            .map(|s| {
                let mut s = str_to_int_range(s);
                s.sort();
                s
            })
            .collect_tuple()
            .unwrap();

        if first_pair.iter().any(|x| second_pair.contains(x))
            || second_pair.iter().any(|x| first_pair.contains(x))
        {
            total_pairs_contained += 1;
        }
    }

    total_pairs_contained
}

fn str_to_int_range(slice: &str) -> Vec<i32> {
    let numbers = slice
        .split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect_vec();

    if let Some(min) = numbers.iter().min() {
        if let Some(max) = numbers.iter().max() {
            (*min..=*max).collect()
        } else {
            panic!("shouldn't be here if input is right");
        }
    } else {
        panic!("possible empty line")
    }
}
