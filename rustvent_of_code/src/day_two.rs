use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut _linecount = 1;
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

        let mut _opponent_a = false;
        let mut _opponent_b = false;
        let mut _opponent_c = false;

        //set outcome of round
        match &input[1] as &str {
            "X" => (score += 0),
            "Y" => score += 1,
            "Z" => score += 3,
            _ => panic!("Should not happen"),
        }

        match &input[1] as &str {
            "X" => loose = true,
            "Y" => draw = true,
            "Z" => win = true,
            _ => panic!("Should not happen"),
        }

        match &input[0] as &str {
            "A" => _opponent_a = true,
            "B" => _opponent_b = true,
            "C" => _opponent_c = true,
            _ => panic!("Should not happen"),
        }
        //identify shape to choose
        //based on opponent & wanted outcome
        //choose paper if A && win / B && draw / C && loose
        //choose Scissors if A && loose / B && win / C && draw
        //choose rock if A && draw / B && loose / C && win

        if _opponent_a && win || _opponent_b && draw || _opponent_c && loose {
            //choose paper & score 2 points
            score += 2;
        } else if _opponent_a && loose || _opponent_b && win || _opponent_c && draw {
            //choose rock & score 3 points
            score += 3;
        } else if _opponent_a && draw || _opponent_b && loose || _opponent_c && win {
            //choose rock & score 1 point
            score += 1;
        }
    }
    println!("Stage 2 Score: {}", score);
}
