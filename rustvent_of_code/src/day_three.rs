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
    for line in file.lines(){
        
    }
}

pub fn part_two() {}
