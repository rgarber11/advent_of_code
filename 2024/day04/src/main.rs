use core::panic;
use day4::Settings;
use std::env;
fn main() {
    let settings = Settings::parse_args(env::args()).expect("Could not parse the cli args.");
    let lines = {
        if let Some(x) = day4::read_lines(&settings.file_path) {
            x
        } else {
            panic!("Could not read file given.")
        }
    };
    let ans = if settings.is_part_two {
        day4::part2(lines).expect("You fucked up")
    } else {
        day4::part1(lines).expect("You fucked up")
    };
    println!("The answer is: {}", ans);
}
