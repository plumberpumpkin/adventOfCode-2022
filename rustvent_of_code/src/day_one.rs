use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn part_one() {
    //TODO read input from input file
    //TODO create list of elve touples (number of elve, calories carried)
    //return elve number & calorie count

    const INPUT_FILE: &str = "./inputs/day_one.txt";

    //initialize intermediate variables & output list
    let mut elf_number: u32 = 1;
    let mut calories_carried: u32 = 0;
    let mut elf_list: Vec<(u32, u32)> = Vec::new();

    let mut file = match File::open(&INPUT_FILE) {
        Err(why) => panic!("couldn't open {}: {}", Path::new(&INPUT_FILE).display(), why),
        Ok(file) => file,
    };

    
}
pub fn part_two() {}
