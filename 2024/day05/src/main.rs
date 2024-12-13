use std::env;

use day5::{FileData, Settings};

mod parts;
fn main() {
    let settings = Settings::parse_args(env::args()).expect("Couldn't parse arguments");
    let data = FileData::read_file(&settings.file_path).expect("Couldn't parse input file");
    let ans: i32 = if settings.is_part_two {
        data.manual_orders
            .iter()
            .map(|order| parts::part2_per_order(data.edges.as_ref(), order.as_ref()))
            .sum()
    } else {
        data.manual_orders
            .iter()
            .map(|order| parts::part1_per_order(data.edges.as_ref(), order.as_ref()))
            .sum()
    };
    println!("The answer is {}", ans);
}
