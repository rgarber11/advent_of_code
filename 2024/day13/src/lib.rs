use core::f64;
use std::{
    env::Args,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
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
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day13/input"),
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
#[derive(PartialEq, Eq)]
enum ParserStates {
    ButtonA,
    ButtonB,
    Prize,
    Blank,
}
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    goal: (i64, i64),
}
pub struct FileData {
    machines: Box<[ClawMachine]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let file = File::open(file_path).ok()?;
        let lines = BufReader::new(file).lines();
        let mut machines = Vec::new();
        let mut button_a = (0, 0);
        let mut button_b = (0, 0);
        let mut goal = (0, 0);
        let mut parser_state = ParserStates::ButtonA;
        for line in lines.into_iter() {
            let line_vec: Vec<String> = line
                .ok()?
                .split_whitespace()
                .map(|s| String::from_str(s).unwrap())
                .collect();
            match parser_state {
                ParserStates::ButtonA => {
                    if line_vec[1] != "A:" {
                        return None;
                    }
                    button_a = (
                        line_vec[2][1..(line_vec[2].len() - 1)].parse().unwrap(),
                        line_vec[3][1..].parse().unwrap(),
                    );
                    parser_state = ParserStates::ButtonB;
                }
                ParserStates::ButtonB => {
                    if line_vec[1] != "B:" {
                        return None;
                    }
                    button_b = (
                        line_vec[2][1..(line_vec[2].len() - 1)].parse().unwrap(),
                        line_vec[3][1..].parse().unwrap(),
                    );
                    parser_state = ParserStates::Prize;
                }
                ParserStates::Prize => {
                    if line_vec[0] != "Prize:" {
                        return None;
                    }
                    goal = (
                        line_vec[1][2..(line_vec[1].len() - 1)].parse().unwrap(),
                        line_vec[2][2..].parse().unwrap(),
                    );
                    parser_state = ParserStates::Blank;
                }
                ParserStates::Blank => {
                    if !line_vec.is_empty() {
                        return None;
                    }
                    machines.push(ClawMachine {
                        button_a,
                        button_b,
                        goal,
                    });
                    parser_state = ParserStates::ButtonA;
                }
            }
        }
        if parser_state == ParserStates::Blank {
            machines.push(ClawMachine {
                button_a,
                button_b,
                goal,
            });
        }
        Some(FileData {
            machines: machines.into_boxed_slice(),
        })
    }
}
fn solve_machine(claw_machine: &ClawMachine) -> Option<(i64, i64)> {
    let determinant_a = (claw_machine.button_a.0 * claw_machine.button_b.1
        - claw_machine.button_b.0 * claw_machine.button_a.1) as f64;
    if determinant_a == 0.0 {
        return None;
    }
    let determinant_a1 = claw_machine.goal.0 * claw_machine.button_b.1
        - claw_machine.button_b.0 * claw_machine.goal.1;
    let determinant_a2 = claw_machine.button_a.0 * claw_machine.goal.1
        - claw_machine.goal.0 * claw_machine.button_a.1;
    let ans_1 = (determinant_a1 as f64) / determinant_a;
    let ans_2 = (determinant_a2 as f64) / determinant_a;
    if ans_1 != ans_1.trunc() || ans_2 != ans_2.trunc() {
        return None;
    }
    Some((ans_1 as i64, ans_2 as i64))
}
pub fn part1(file_data: FileData) -> i64 {
    file_data
        .machines
        .iter()
        .map(|claw| {
            if let Some(ans) = solve_machine(claw) {
                if ans.0 > 100 || ans.1 > 100 {
                    0
                } else {
                    ans.0 * 3 + ans.1 * 1
                }
            } else {
                0
            }
        })
        .sum()
}
pub fn part2(mut file_data: FileData) -> i64 {
    file_data
        .machines
        .iter_mut()
        .map(|claw| {
            claw.goal = (claw.goal.0 + 10000000000000, claw.goal.1 + 10000000000000);
            if let Some(ans) = solve_machine(claw) {
                ans.0 * 3 + ans.1 * 1
            } else {
                0
            }
        })
        .sum()
}
