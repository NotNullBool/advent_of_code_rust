use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let answer1 = first_marker_detect(input);
    println!("{answer1:?} first answer!");
}

fn first_marker_detect(input: String) -> usize {
    let temp = input.chars().collect::<Vec<_>>();
    let four_at_a_time = temp.windows(4);

    'main: for (main_index, four_chars) in four_at_a_time.enumerate() {
        for (i, char1) in four_chars.iter().enumerate() {
            for (j, char2) in four_chars.iter().enumerate() {
                if i == j {
                    continue;
                }
                if char1 == char2 {
                    continue 'main;
                }
            }
            if i == four_chars.len() - 1 {
                return main_index + 4;
            }
        }
    }

    panic!("input file isnt set up properly");
}
