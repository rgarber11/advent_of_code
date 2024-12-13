use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    env::Args,
    fs::File,
    io::{BufRead, BufReader},
};
pub struct Settings {
    pub is_part_two: bool,
    pub file_path: Box<str>,
}
impl Settings {
    pub fn parse_args(mut args: Args) -> Option<Settings> {
        match args.len() {
            1 => Some(Settings {
                is_part_two: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day10/input"),
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
    topology: Box<[Box<[i8]>]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let file = File::open(file_path).ok()?;
        let lines = BufReader::new(file).lines();
        let topology: Box<[Box<[i8]>]> = lines
            .into_iter()
            .map(|res| {
                let str = res.expect("Line must exist for function to work");
                if !str.is_ascii() {
                    panic!("We cannot deal with non-ascii characters");
                }
                let line: Box<[i8]> = str
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .expect("All characters in the line should be digits")
                            as i8
                    })
                    .collect();
                line
            })
            .collect();
        Some(FileData { topology })
    }
}
fn trail_score(init: (usize, usize), field: &[Box<[i8]>]) -> i64 {
    let mut visited_nines: HashSet<(usize, usize)> = HashSet::new();
    let mut visit_list: VecDeque<(usize, usize)> = VecDeque::from([init]);
    let line_len = field[0].len();
    while !visit_list.is_empty() {
        let curr = visit_list.pop_front().unwrap();
        let curr_val = field[curr.0][curr.1];
        if curr.0 != 0 && field[curr.0 - 1][curr.1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                visited_nines.insert((curr.0 - 1, curr.1));
            } else {
                visit_list.push_back((curr.0 - 1, curr.1));
            }
        }
        if curr.0 + 1 < field.len() && field[curr.0 + 1][curr.1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                visited_nines.insert((curr.0 + 1, curr.1));
            } else {
                visit_list.push_back((curr.0 + 1, curr.1));
            }
        }
        if curr.1 != 0 && field[curr.0][curr.1 - 1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                visited_nines.insert((curr.0, curr.1 - 1));
            } else {
                visit_list.push_back((curr.0, curr.1 - 1));
            }
        }
        if curr.1 + 1 < line_len && field[curr.0][curr.1 + 1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                visited_nines.insert((curr.0, curr.1 + 1));
            } else {
                visit_list.push_back((curr.0, curr.1 + 1));
            }
        }
    }
    visited_nines.len() as i64
}
fn trail_rating(init: (usize, usize), field: &[Box<[i8]>]) -> i64 {
    let mut ans = 0;
    let mut visit_list: VecDeque<(usize, usize)> = VecDeque::from([init]);
    let line_len = field[0].len();
    while !visit_list.is_empty() {
        let curr = visit_list.pop_front().unwrap();
        let curr_val = field[curr.0][curr.1];
        if curr.0 != 0 && field[curr.0 - 1][curr.1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                ans += 1;
            } else {
                visit_list.push_back((curr.0 - 1, curr.1));
            }
        }
        if curr.0 + 1 < field.len() && field[curr.0 + 1][curr.1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                ans += 1;
            } else {
                visit_list.push_back((curr.0 + 1, curr.1));
            }
        }
        if curr.1 != 0 && field[curr.0][curr.1 - 1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                ans += 1;
            } else {
                visit_list.push_back((curr.0, curr.1 - 1));
            }
        }
        if curr.1 + 1 < line_len && field[curr.0][curr.1 + 1] == curr_val + 1 {
            if curr_val + 1 == 9 {
                ans += 1;
            } else {
                visit_list.push_back((curr.0, curr.1 + 1));
            }
        }
    }
    ans
}
pub fn part1(file_data: FileData) -> i64 {
    let line_len = file_data.topology[0].len();
    let mut ans = 0;
    for i in 0..file_data.topology.len() {
        for j in 0..line_len {
            if file_data.topology[i][j] == 0 {
                ans += trail_score((i, j), file_data.topology.as_ref());
            }
        }
    }
    ans
}
pub fn part2(file_data: FileData) -> i64 {
    let line_len = file_data.topology[0].len();
    let mut ans = 0;
    for i in 0..file_data.topology.len() {
        for j in 0..line_len {
            if file_data.topology[i][j] == 0 {
                ans += trail_rating((i, j), file_data.topology.as_ref());
            }
        }
    }
    ans
}
