use std::io::BufReader;

use std::io::BufRead;
use std::fs::File;


pub fn part_one() {
    const MOVING_INSTRUCTIONS: &str = r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\moving_instructions.txt";
    const STARTING_POINT: &str = r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\starting_point_day_five.txt";
    let mut supply_stack: Vec<Vec<char>> = Vec::new();
    let start: BufReader<File> = super::utils::read_input_file(STARTING_POINT);
    for stack in start.lines(){
        let current_stack: Vec<char> = stack.unwrap().split(";").map(|x| x.parse::<char>().unwrap()).collect();
        supply_stack.push(current_stack);

    }
    let movement_set = super::utils::read_input_file(MOVING_INSTRUCTIONS);
    //deconstruct moving instructions
}

pub fn part_two() {}
