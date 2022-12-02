use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn part_one() {
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_one.txt";

    let mut current_elf: (u32, u32) = (0, 0);
    let mut elf_number: u32 = 1;
    let mut calories_carried: u32 = 0;
    let mut elf_list: Vec<(u32, u32)> = Vec::new();

    let file = match File::open(&INPUT_FILE) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            Path::new(&INPUT_FILE).display(),
            why
        ),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap() != "" {
            let item: u32 = line.as_ref().unwrap().parse().unwrap();
            calories_carried += item;
        } else if line.as_ref().unwrap() == "" {
            current_elf.0 = elf_number;
            current_elf.1 = calories_carried;
            elf_list.push(current_elf);
            elf_number += 1;
            calories_carried = 0;
        }
    }

    let mut max_calories: u32 = 0;
    let mut target_elf: u32 = 0;
    for item in elf_list {
        if max_calories < item.1 {
            target_elf = item.0;
            max_calories = item.1;
        }
    }
    println!(
        "Elf no. {} carried most with {} calories in total",
        target_elf, max_calories
    );
}
pub fn part_two() {
    const INPUT_FILE: &str =
        r"C:\Users\User\GitHub\adventOfCode-2022\rustvent_of_code\src\inputs\day_one.txt";

    let mut current_elf: (u32, u32) = (0, 0);
    let mut elf_number: u32 = 1;
    let mut calories_carried: u32 = 0;
    let mut elf_list: Vec<(u32, u32)> = Vec::new();

    let file = match File::open(&INPUT_FILE) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            Path::new(&INPUT_FILE).display(),
            why
        ),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.as_ref().unwrap() != "" {
            let item: u32 = line.as_ref().unwrap().parse().unwrap();
            calories_carried += item;
        } else if line.as_ref().unwrap() == "" {
            current_elf.1 = elf_number;
            current_elf.0 = calories_carried;
            elf_list.push(current_elf);
            elf_number += 1;
            calories_carried = 0;
        }
    }
    elf_list.sort();
    println!(
        "1. Place: {}, {}",
        elf_list[elf_list.len() - 1].0,
        elf_list[elf_list.len() - 1].1
    );
    println!(
        "2. Place: {}, {}",
        elf_list[elf_list.len() - 2].0,
        elf_list[elf_list.len() - 2].1
    );
    println!(
        "3. Place: {}, {}",
        elf_list[elf_list.len() - 3].0,
        elf_list[elf_list.len() - 3].1
    );
    let total_snack_reserve: u32 = elf_list[elf_list.len() - 1].0
        + elf_list[elf_list.len() - 2].0
        + elf_list[elf_list.len() - 3].0;

    println!(
        "The total snack reserve contains {} caloires",
        total_snack_reserve
    );
}
