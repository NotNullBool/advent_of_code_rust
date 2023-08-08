use std::{fs::read_to_string, io::Error};

//Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn main() {
    let mut total_elf_calories = input_file_to_elves_total_calories("./input.txt").unwrap();

    total_elf_calories.sort();
    total_elf_calories.reverse();

    println!("{:#?} is the top elf's calories.", total_elf_calories[0]);
    //part 2

    println!(
        "{:#?} is the combined top 3 elves calories.",
        total_elf_calories[0..=2].iter().sum::<i32>()
    );
}

//Take input file of path then combines all numbers together until empty line then returns a vector
//of the individual elves
fn input_file_to_elves_total_calories(file_name: &str) -> Result<Vec<i32>, Error> {
    match read_to_string(file_name) {
        Ok(input) => {
            let mut totals = vec![];
            let mut index = 0;
            for line in input.lines() {
                //if empty line means the next elf
                if line.is_empty() {
                    index += 1;
                    continue;
                } else {
                    let curr_line_num = line.parse::<i32>().unwrap();
                    if totals.len() <= index {
                        totals.push(curr_line_num)
                    } else {
                        totals[index] += curr_line_num;
                    }
                }
            }
            Ok(totals)
        }
        Err(e) => {
            println!("please put input.txt file in executable directory");
            Err(e)
        } //file doesnt exist in current directory
    }
}
