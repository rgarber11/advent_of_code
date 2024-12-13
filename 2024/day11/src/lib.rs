use std::{collections::HashMap, env::Args, fs::File, io::Read};

pub struct Settings {
    pub is_part_two: bool,
    pub file_path: Box<str>,
}
impl Settings {
    pub fn parse_args(mut args: Args) -> Option<Settings> {
        match args.len() {
            1 => Some(Settings {
                is_part_two: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day11/input"),
            }),
            2 => Some(Settings {
                is_part_two: false,
                file_path: unsafe { Box::from(args.nth(1).unwrap_unchecked()) },
            }),
            3 => Some(Settings {
                is_part_two: match unsafe { args.nth(1).unwrap_unchecked().as_str() } {
                    "1" => false,
                    "2" => true,
                    _ => panic!("Only parts 1 or 2 are options"),
                },
                file_path: unsafe { Box::from(args.nth(0).unwrap_unchecked()) },
            }),
            _ => None,
        }
    }
}
pub struct FileData {
    pub initial_line: Box<[i64]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let mut file_str = String::new();
        File::open(file_path)
            .ok()?
            .read_to_string(&mut file_str)
            .ok()?;
        let initial_line: Box<[i64]> = file_str
            .trim()
            .split_whitespace()
            .map(|num| num.parse().expect("Each element of string should be num"))
            .collect();
        Some(FileData { initial_line })
    }
}
fn single_rock_rule(rock: i64) -> [i64; 2] {
    match rock {
        0 => [1, -1],
        x if ((x as f64).log10().floor() as i64) % 2 == 1 => {
            let num_digits = (x as f64).log10().floor() as i64 + 1;
            let mut first_digit = x;
            let mut second_digit = 0;
            for i in 0..(num_digits / 2) {
                second_digit += (first_digit % 10) * 10i64.pow(i as u32);
                first_digit /= 10;
            }
            [first_digit, second_digit]
        }
        _ => [rock * 2024, -1],
    }
}
fn get_rock_out_after_n_iterations(
    rock: i64,
    n: usize,
    cache: &mut HashMap<usize, HashMap<i64, i64>>,
) -> i64 {
    match cache.entry(n).or_default().get(&rock) {
        Some(x) => x.clone(),
        None => match n {
            1 => {
                let res = single_rock_rule(rock);
                let ans = res.into_iter().filter(|i| *i != -1).count() as i64;
                cache.entry(n).or_default().insert(rock, ans);
                ans
            }
            _ => {
                let ans = single_rock_rule(rock)
                    .into_iter()
                    .filter_map(|new_rock| match new_rock {
                        -1 => None,
                        _ => Some(get_rock_out_after_n_iterations(new_rock, n - 1, cache)),
                    })
                    .sum();
                cache.entry(n).or_default().insert(rock, ans);
                ans
            }
        }
        .clone(),
    }
}
pub fn part1(file_data: FileData) -> i64 {
    let mut cache: HashMap<usize, HashMap<i64, i64>> = HashMap::with_capacity(25);
    let ans = file_data
        .initial_line
        .iter()
        .map(|rock| get_rock_out_after_n_iterations(*rock, 25, &mut cache))
        .sum();
    ans
}
pub fn part2(file_data: FileData) -> i64 {
    let mut cache: HashMap<usize, HashMap<i64, i64>> = HashMap::with_capacity(75);
    let ans = file_data
        .initial_line
        .iter()
        .map(|rock| get_rock_out_after_n_iterations(*rock, 75, &mut cache))
        .sum();
    ans
}
