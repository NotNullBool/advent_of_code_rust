use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let answer1 = first_marker_detect(input.clone(), 4);
    let answer2 = first_marker_detect(input, 14);

    println!("{answer1:?} first answer!\n{answer2:?} second answer!");
}

fn first_marker_detect(input: String, how_many_unqiue_chars: usize) -> usize {
    let temp = input.chars().collect::<Vec<_>>();
    let four_at_a_time = temp.windows(how_many_unqiue_chars);

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
                return main_index + how_many_unqiue_chars;
            }
        }
    }

    panic!("input file isnt set up properly");
}
