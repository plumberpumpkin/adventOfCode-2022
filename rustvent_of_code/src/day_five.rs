use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;


pub fn part_one() {
    const MOVING_INSTRUCTIONS: &str = r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\moving_instructions.txt";
    const STARTING_POINT: &str = r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\starting_point_day_five.txt";
    let mut supply_stack: Vec<Vec<char>> = Vec::new();
    let start: BufReader<File> = super::utils::read_input_file(STARTING_POINT);
    //reading in input matrix
    for stack in start.lines() {
        let current_stack: Vec<char> = stack
            .unwrap()
            .split(";")
            .map(|x| x.parse::<char>().unwrap())
            .collect();
        supply_stack.push(current_stack);
    }
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    let movement_set = super::utils::read_input_file(MOVING_INSTRUCTIONS);
    //deconstruct moving instructions
    for instruction in movement_set.lines() {
        let current_instruction = instruction.unwrap();
        let mut work_input_str: Vec<String> = current_instruction
            .split(" ")
            .map(|x| x.parse::<String>().unwrap())
            .collect();
        //remove strings for further utilization
        work_input_str.remove(4);
        work_input_str.remove(2);
        work_input_str.remove(0);
        //convert to int value
        let work_input: Vec<usize> = work_input_str.iter().map(|x| x.parse().unwrap()).collect();
        
        instructions.push(work_input);
    }
    //TODO apply instructions
    for step in instructions {
        let move_count = step[0];
        
            

        


    }

}

pub fn part_two() {}
