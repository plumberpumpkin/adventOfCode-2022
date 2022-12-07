use std::io::BufRead;

pub fn part_one() {
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_four.txt";
    let file = super::utils::read_input_file(INPUT_FILE);

    //initialize relevant variables
    let mut containing_counter: u32 = 0;

    for line in file.lines() {
        let intermediate: String = line.unwrap();
        let assignments: Vec<&str> = intermediate.split(",").collect();
        let range_one: Vec<u32> = assignments[0]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let range_two: Vec<u32> = assignments[1]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        //case range one contains range two
        //start two >=start one && end two <=end one
        if range_two[0] >= range_one[0] && range_two[1] <= range_one[1] {
            containing_counter += 1;
        } else if range_one[0] >= range_two[0] && range_one[1] <= range_two[1] {
            containing_counter += 1;
        }

        //case range two contains range one
        //start one >=start two && end one <=end two
    }
    println!("No of contained assignements: {}", containing_counter);
}

pub fn part_two() {
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_four.txt";
    let file = super::utils::read_input_file(INPUT_FILE);

    let mut overlapping_assignments: u32 = 0;

    for line in file.lines() {
        let intermediate: String = line.unwrap();
        let assignments: Vec<&str> = intermediate.split(",").collect();
        let range_one: Vec<u32> = assignments[0]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let range_two: Vec<u32> = assignments[1]
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        //case assignment one reaches in assigment two
        //1-3 & 2-5
        //case assignment two reaches in assignment one
        //2-5 && 1-3
        // one fully contained in two
        // two fully contained in one
        if (range_two[0] >= range_one[0] && range_two[1] <= range_one[1])
            || (range_one[0] >= range_two[0] && range_one[1] <= range_two[1])
            || (range_one[0] <= range_two[0] && range_one[1] >= range_two[0])
            || (range_two[0] <= range_one[0] && range_two[1] >= range_one[0])
        {
            overlapping_assignments += 1;
        }
    }
    println!(
        "No. of overlapping assignments: {}",
        overlapping_assignments
    );
}
