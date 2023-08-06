use std::{char, fs::read_to_string, u8};

fn main() {
    let input = input_file_to_string("./input.txt").unwrap();
    let answer1 = rugsack_priority_sum(input.clone());
    let answer2 = rugsack_priority_sum_three_set(input);

    println!("{:#?} is the answer to part 1", answer1);
    println!("{:#?} is the answer to part 2", answer2)
}

fn input_file_to_string(filepath: &str) -> Result<String, &str> {
    match read_to_string(filepath) {
        Ok(input) => Ok(input),
        Err(_) => Err("Please put input.txt in executable directory."),
    }
}

fn alphabet_to_i32(character: char) -> i32 {
    return match character {
        'a'..='z' => (character as u8 - b'a') + 1,
        'A'..='Z' => (character as u8 - b'A') + 27,
        _ => 0,
    } as i32;
}

fn rugsack_priority_sum(input: String) -> i32 {
    let mut line_priority: Vec<i32> = Vec::new();

    for line in input.lines() {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        for character in second_half.chars() {
            if first_half.contains(character) {
                line_priority.push(alphabet_to_i32(character));
                break;
            }
        }
    }

    line_priority.into_iter().sum()
}

fn rugsack_priority_sum_three_set(input: String) -> i32 {
    let mut line_priority: Vec<i32> = Vec::new();
    let mut three_set_lines: [Option<&str>; 3] = [None; 3];

    for cur_line in input.lines() {
        for (index, opt) in three_set_lines.iter().enumerate() {
            match opt {
                Some(_) => continue,
                None => {
                    three_set_lines[index] = Some(cur_line);
                    break;
                }
            }
        }

        match three_set_lines[2] {
            Some(last_line) => {
                let common_char = last_line
                    .chars()
                    .filter(|c| three_set_lines[1].unwrap().contains(*c))
                    .filter(|c| three_set_lines[0].unwrap().contains(*c))
                    .next()
                    .unwrap();
                line_priority.push(alphabet_to_i32(common_char));

                three_set_lines = [None; 3];
            }
            None => (),
        }
    }
    line_priority.into_iter().sum()
}
