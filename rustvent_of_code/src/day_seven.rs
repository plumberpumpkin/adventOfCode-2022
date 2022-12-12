use std::io::BufRead;

pub fn part_one() {
    const INPUT: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_seven.txt";
    let puzzleinput = super::utils::read_input_file(INPUT);

    //initialize result variables
    let mut current_directory: String = String::from("");
    let mut current_directory_size: u32 = 0;

    let mut resulting_size: u32 = 0;

    for line in puzzleinput.lines() {
        let current_line = line.unwrap().clone();
        //identify if command or output
        // if starts with $ => command, else => output line
        if current_line.starts_with("$") {
            //command section
            let parts: Vec<&str> = current_line.split(" ").collect();
            match parts[1] {
                "cd" => {
                    if parts[2] != ".." {
                        current_directory += parts[2]
                    } else if parts[2] == ".." {
                        //TODO add remove current folder from current directory
                    }
                }
                "ls" => println!("Current directory is {}", current_directory),
                &_ => println!("Should not happen"),
            }
        } else {
            //output section
        }
    }
}

pub fn part_two() {}
