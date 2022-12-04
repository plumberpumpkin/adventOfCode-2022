use std::io::BufRead;

pub fn part_one() {
    //define input location
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_three.txt";
    let file = super::utils::read_input_file(INPUT_FILE);

    //read single lines from file
    //split line in half
    //find char that is part of both halfs
    //push char in intermediate vector

    //initilize intermediate vector
    let mut relevant_chars: Vec<char> = Vec::new();
    for line in file.lines() {
        let rucksack_content: String = line.unwrap();
        let compartment = rucksack_content.split_at(rucksack_content.len() / 2);

        //iterate over first compartment to find item of same type in the second compartment
        for item in compartment.0.chars() {
            let index = compartment.1.find(item);
            if index != None {
                relevant_chars.push(item);
                break;
            }
        }
    }
    //calculate priorities based on characters
    let mut priorities: u32 = 0;
    for item in relevant_chars {
        let mut prio = item as u32;

        if item.is_ascii_uppercase() {
            //priorities of uppercase starts at 27 (vs. ascii codepoint 65 for A)
            prio -= 38;
        } else {
            prio -= 96;
        }
        priorities += prio;
    }

    println!("Total sum of priorities is {}", priorities);
}

pub fn part_two() {}
