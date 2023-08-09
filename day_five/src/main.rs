use std::fs::read_to_string;

//TODO:refactor commonalities of part 1 and 2
fn main() {
    let mut arr: [Vec<char>; 10] = [
        vec![], //empty vector just to make input easier for me
        vec!['Q', 'M', 'G', 'C', 'L'],
        vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
        vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
        vec!['J', 'F', 'D', 'V', 'Q', 'P'],
        vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
        vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
        vec!['H', 'C', 'T'],
        vec!['G', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
        vec!['Z', 'F', 'H', 'G'],
    ];

    let input = read_to_string("./input.txt").ok().unwrap();
    let answer1 = crate_organize(input.clone(), &mut arr.clone());
    let answer2 = crate_organize_part_2(input, &mut arr);
    println!("{answer1:?} is part 1 {answer2:?} is part 2");
}

fn crate_organize_part_2(input: String, arr: &mut [Vec<char>; 10]) -> [char; 9] {
    for line in input.lines() {
        let (_, main) = line.split_once("move ").unwrap();
        let (movement, main) = main.split_once(" from ").unwrap();
        let (from, to) = main.split_once(" to ").unwrap();
        let movement: usize = movement.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        let from_len = arr[from].len();
        let val: Vec<_> = arr[from].drain(from_len - movement..from_len).collect();
        arr[to].extend(val.into_iter());
    }

    let mut arr_chars: [Option<char>; 9] = [None; 9];

    for (index, cur_vec) in arr.iter_mut().enumerate() {
        if index == 0 {
            continue;
        }
        arr_chars[index - 1] = cur_vec.pop();
    }

    arr_chars.map(|c| c.unwrap())
}

fn crate_organize(input: String, arr: &mut [Vec<char>; 10]) -> [char; 9] {
    for line in input.lines() {
        let (_, main) = line.split_once("move ").unwrap();
        let (movement, main) = main.split_once(" from ").unwrap();
        let (from, to) = main.split_once(" to ").unwrap();
        let movement: usize = movement.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        let from_len = arr[from].len();
        for _ in from_len - movement..from_len {
            let val = arr[from].pop().unwrap();
            arr[to].push(val);
        }
    }

    let mut arr_chars: [Option<char>; 9] = [None; 9];

    for (index, cur_vec) in arr.iter_mut().enumerate() {
        if index == 0 {
            continue;
        }
        arr_chars[index - 1] = cur_vec.pop();
    }

    arr_chars.map(|c| c.unwrap())
}
