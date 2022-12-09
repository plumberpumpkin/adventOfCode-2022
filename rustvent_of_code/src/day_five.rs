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
        let moving_stack_size = step[0];
        let start_stack_number = step[1] - 1;
        let target_stack_number = step[2] - 1;
        let mut start_stack: Vec<char> = supply_stack[start_stack_number].clone();
        let mut target_stack: Vec<char> = supply_stack[target_stack_number].clone();
        let mut moves_done: usize = 0;

        while moves_done < moving_stack_size {
            let item = start_stack[0];
            target_stack.insert(0, item);
            start_stack.remove(0);
            moves_done += 1;
        }

        let target_stack_to_remove = target_stack_number + 1;
        let start_stack_to_remove = start_stack_number + 1;
        //replace targetstack
        supply_stack.insert(target_stack_number, target_stack);
        supply_stack.remove(target_stack_to_remove);
        //replace startstack
        supply_stack.insert(start_stack_number, start_stack);
        supply_stack.remove(start_stack_to_remove);
    }

    //Create message for elfs
    let mut first_crates: String = String::new();
    let mut count = 0;
    for stack in supply_stack {
        let current_crate = stack[0];
        first_crates.insert(count, current_crate);
        count += 1;
    }
    println!("Part 1: First crates are: {}", first_crates);
}

pub fn part_two() {
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

    let mut current_step = 0;

    for step in instructions {
        current_step += 1;
        let moving_stack_size_index = step[0] - 1;
        let start_stack_number = step[1] - 1;
        let target_stack_number = step[2] - 1;
        let mut start_stack: Vec<char> = supply_stack[start_stack_number].clone();
        let mut target_stack: Vec<char> = supply_stack[target_stack_number].clone();
        

        //apply instructions
        //move multiple items at a time according to instruction
        if start_stack.len() > moving_stack_size_index {
            let sub_stacks = start_stack.split_at(moving_stack_size_index);
        let mut moving_items = sub_stacks.0.to_vec();
        moving_items.reverse();
        for item in moving_items {
            target_stack.insert(0, item);
        }
        start_stack = sub_stacks.1.to_vec();
        } else {
            println!("Attempt ({}) is larger than stack ({}), step is {} ", step[0], start_stack.len(), current_step);
        }
        




        let target_stack_to_remove = target_stack_number + 1;
        let start_stack_to_remove = start_stack_number + 1;
        //replace targetstack
        supply_stack.insert(target_stack_number, target_stack);
        supply_stack.remove(target_stack_to_remove);
        //replace startstack
        supply_stack.insert(start_stack_number, start_stack);
        supply_stack.remove(start_stack_to_remove);
    }


    //Create message for elfs
    let mut first_crates: String = String::new();
    let mut count = 0;
    for stack in supply_stack {
        let current_crate = stack[0];
        first_crates.insert(count, current_crate);
        count += 1;
    }
    println!("Part 2: First crates are: {}", first_crates);
}
