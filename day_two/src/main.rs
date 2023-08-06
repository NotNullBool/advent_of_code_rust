use std::fs::read_to_string;

//Refactor to add constants to make letters more understandable

fn main() {
    let input_string = input_as_string("./input.txt").unwrap();
    let answer = rock_paper_scissors_strat_part_1(input_string.clone());

    let answer2 = rock_paper_scissors_strat_part_2(input_string);

    println!("{:#?} solution part 1", answer);
    println!("{:#?} solution part 2", answer2);
}

fn input_as_string(filepath: &str) -> Result<String, &str> {
    match read_to_string(filepath) {
        Ok(input) => Ok(input),
        Err(_) => Err("Please put input.txt file in executable directory"),
    }
}

fn rock_paper_scissors_strat_part_2(input: String) -> i32 {
    let mut total_score = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "A" => {
                match parts[1] {
                    "X" => total_score += 3, //Scissors + Lost
                    "Y" => total_score += 4, //Rock + Draw
                    "Z" => total_score += 8, //Paper + Win
                    _ => (),
                }
            }
            "B" => {
                match parts[1] {
                    "X" => total_score += 1, //Rock + Lost
                    "Y" => total_score += 5, //Paper + Draw
                    "Z" => total_score += 9, //Scissors + Win
                    _ => (),
                }
            }
            "C" => {
                match parts[1] {
                    "X" => total_score += 2, //Paper + Lost
                    "Y" => total_score += 6, //Scissors + Draw
                    "Z" => total_score += 7, //Rock + Win
                    _ => (),
                }
            }
            _ => (),
        }
    }

    return total_score;
}

//If X Y Z meant rock paper scissors
fn rock_paper_scissors_strat_part_1(input: String) -> i32 {
    let mut total_score = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[1] {
            "X" => {
                total_score += 1;
                match parts[0] {
                    "A" => total_score += 3, //Draw
                    "B" => total_score += 0, //Lost
                    "C" => total_score += 6, //Win
                    _ => (),
                }
            }
            "Y" => {
                total_score += 2;
                match parts[0] {
                    "A" => total_score += 6, //Win
                    "B" => total_score += 3, //Draw
                    "C" => total_score += 0, //Lost
                    _ => (),
                }
            }
            "Z" => {
                total_score += 3;
                match parts[0] {
                    "A" => total_score += 0, //Lost
                    "B" => total_score += 6, //Win
                    "C" => total_score += 3, //Draw
                    _ => (),
                }
            }
            _ => (),
        }
    }

    total_score
}
