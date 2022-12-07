use std::io::BufRead;

pub fn part_one(){

    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_four.txt";
    let file = super::utils::read_input_file(INPUT_FILE);

    //initialize relevant variables
    let mut containing_counter: u32 = 0;

    for line in file.lines(){
        let assignments: Vec<&str> = line.unwrap().split(",").collect();
        
        
        
    }


}

pub fn part_two(){

}