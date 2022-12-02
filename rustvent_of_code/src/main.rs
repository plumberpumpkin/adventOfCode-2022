mod day_one;
mod day_two;

fn main() {
    //here come the inputs for the
    let mut day: u16 = 1;

    println!("Advent of Code - Day {}", day);
    day_one::part_one();
    day_one::part_two();
    day += 1;

    println!("Advent of Code - Day {}", day);
    day_two::part_one();
    day_two::part_two();
    day += 1;

    println!("Advent of Code - Day {}", day);
    day += 1;

    println!("Advent of Code - Day {}", day);
    day += 1;

    println!("Advent of Code - Day {}", day);
    day += 1;

    println!("Advent of Code - Day {}", day);
    day += 1;
}
