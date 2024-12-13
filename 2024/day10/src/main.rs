use std::env;

use day10::{FileData, Settings};

fn main() {
    let settings = Settings::parse_args(env::args()).expect("Couldn't parse args.");
    let file_data = FileData::read_file(&settings.file_path).expect("Couldn't read file data.");
    let ans = if settings.is_part_two {
        day10::part2(file_data)
    } else {
        day10::part1(file_data)
    };
    println!("The answer is {}", ans);
}