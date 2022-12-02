use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part_one() {
    //define input file location
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_two.txt";

    //read file

    let file = File::open(&INPUT_FILE).unwrap();
    let reader = BufReader::new(file);

    //read single line and split into two parts
    //initilize result value
    let mut score: u32 = 0;
    let mut linecount = 1;
    for line in reader.lines() {
        let input: Vec<String> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        //compare the two values
        //A/X => Rock
        //B/Y => Paper
        //C/Z => Scissors

        let mut win = false;
        let mut draw = false;
        let mut loose = false;

        //Outcome
        if input[0] == "A" && input[1] == "Y"
            || input[0] == "B" && input[1] == "Z"
            || input[0] == "C" && input[1] == "X"
        {
            win = true;
        } else if input[0] == "A" && input[1] == "X"
            || input[0] == "B" && input[1] == "Y"
            || input[0] == "C" && input[1] == "Z"
        {
            draw = true;
        } else if input[0] == "A" && input[1] == "Z"
            || input[0] == "B" && input[1] == "X"
            || input[0] == "C" && input[1] == "Y"
        {
            loose = true;
        }

        //calculate result for this round
        if win {
            score += 6;
        } else if draw {
            score += 3;
        } else if loose {
            score += 0;
        }

        //add points from shape
        match &input[1] as &str {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => panic!("This should not happen"),
        }
    }

    println!("Your score is: {}", score);
}

pub fn part_two() {
    //define input file location
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_two.txt";

    //read file

    let file = File::open(&INPUT_FILE).unwrap();
    let reader = BufReader::new(file);

    //read single line and split into two parts
    //initilize result value
    let mut score: u32 = 0;
    let mut linecount = 1;
    for line in reader.lines() {
        let input: Vec<String> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        //compare the two values
        //A => Rock
        //B => Paper
        //C => Scissors
        //X => loose
        //Y => draw
        //Z => win

        let mut win = false;
        let mut draw = false;
        let mut loose = false;

        //set outcome of round
        match &input[1] as &str {
            "X" => score += 0,
            "Y" => score += 1,
            "Z" => score += 3,
            _ => panic!("Should not happen"),
        }

       //identify shape to choose 
       //based on opponent & wanted outcome

        
    }
    println!("Stage 2 Score: {}", score);
}
