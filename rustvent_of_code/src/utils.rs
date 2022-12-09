//TODO build generic header line with the current date and stage -> recursion?
use std::fs::File;
use std::io::BufReader;

/* pub fn header(input: (u16, u16)) -> (u16, u16) {
    println!("Advent of Code - Day {}", input.1);
    return (1, 1);
} */

pub fn read_input_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    return reader;
}
