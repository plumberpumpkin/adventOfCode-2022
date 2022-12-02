use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


pub fn part_one() {
    //TODO read input from input file
    //TODO create list of elve touples (number of elve, calories carried)
    //return elve number & calorie count

    const INPUT_FILE: &str = r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_one.txt";

    //initialize intermediate variables & output list
    let mut current_elf: (u32, u32) = (0,0);
    let mut elf_number: u32 = 1;
    let mut calories_carried: u32 = 0;
    let mut elf_list: Vec<(u32, u32)> = Vec::new();

    let file = match File::open(&INPUT_FILE) {
        Err(why) => panic!("couldn't open {}: {}", Path::new(&INPUT_FILE).display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    //println!("{}", INPUT_FILE);
    //read line
    //if line is not blank => add number to amount carried
    //if line is blank => push tupel to array && increment index
    

        
     for line in reader.lines(){
        println!("{:?}", line);
        if line.as_ref().unwrap() != "" {

            let item: u32 = line.as_ref().unwrap().parse().unwrap();
            calories_carried += item;

        } else if line.as_ref().unwrap() == ""{

            current_elf.0 = elf_number;
            
            current_elf.1 = calories_carried;
            elf_list.push(current_elf);
            //println!("Following elf was pushed to list: no.: {}, carried calories: {}", current_elf.0 , current_elf.1);
            elf_number += 1;
            calories_carried = 0;
            //println!("New elf number is {}. Calories reset to value {}", elf_number, calories_carried)
        }
     }

     let mut max_calories:u32 = 0;
     let mut target_elf: u32 = 0;
     for item in elf_list{
        if max_calories < item.1 {
            target_elf = item.0;
            max_calories = item.1;
        }
        //println!("{}, {}", item.0, item.1);
     }
     println!("Elf no. {} carried most with {} calories in total", target_elf, max_calories);


    
}
pub fn part_two() {}
