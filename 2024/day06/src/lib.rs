use std::borrow::BorrowMut;
use std::env::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::IndexMut;
pub struct Settings {
    pub is_part_two: bool,
    pub file_path: Box<str>,
}
impl Settings {
    pub fn parse_args(mut args: Args) -> Option<Settings> {
        match args.len() {
            1 => Some(Settings {
                is_part_two: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day06/input"),
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
#[derive(Clone)]
pub struct FileData {
    pub initial_loc: (usize, usize),
    pub lines: Box<[Box<str>]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let files = File::open(file_path).ok()?;
        let lines: Box<[Box<str>]> = BufReader::new(files)
            .lines()
            .map(|i| Box::from(i.unwrap()))
            .collect();
        if !lines.iter().all(|i| i.is_ascii()) {
            return None;
        }
        let mut inital_loc = (0, 0);
        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.char_indices() {
                if char == '^' {
                    inital_loc = (i.try_into().unwrap(), j.try_into().unwrap());
                }
            }
        }
        Some(FileData {
            initial_loc: inital_loc,
            lines,
        })
    }
}
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
        }
    }
    pub fn to_motion(&self, curr_loc: &mut (usize, usize)) {
        match self {
            Direction::LEFT => {
                curr_loc.1 -= 1;
            }
            Direction::RIGHT => {
                curr_loc.1 += 1;
            }
            Direction::UP => {
                curr_loc.0 -= 1;
            }
            Direction::DOWN => {
                curr_loc.0 += 1;
            }
        };
    }
}
fn done(
    direction: &Direction,
    curr_loc: &(usize, usize),
    num_lines: &usize,
    line_size: &usize,
) -> bool {
    match direction {
        Direction::LEFT => curr_loc.1 == 0,
        Direction::RIGHT => curr_loc.1 == (line_size - 1),
        Direction::UP => curr_loc.0 == 0,
        Direction::DOWN => curr_loc.0 == (num_lines - 1),
    }
}
pub fn part1(mut file_data: FileData) -> i32 {
    let mut curr_dir = Direction::UP;
    let num_lines: usize = file_data.lines.len();
    let line_size: usize = file_data.lines[0].chars().count();
    let mut ans = 0;
    while !done(&curr_dir, &file_data.initial_loc, &num_lines, &line_size) {
        if let Some(x) = file_data.lines[file_data.initial_loc.0]
            .as_mut()
            .chars()
            .nth(file_data.initial_loc.1)
        {
            if x != 'X' {
                ans += 1;
                unsafe {
                    *file_data.lines[file_data.initial_loc.0]
                        .as_bytes_mut()
                        .index_mut(file_data.initial_loc.1) = b'X';
                }
            }
            let prev_motion = file_data.initial_loc.clone();
            curr_dir.to_motion(file_data.initial_loc.borrow_mut());
            if file_data.lines[file_data.initial_loc.0]
                .chars()
                .nth(file_data.initial_loc.1)
                .unwrap()
                == '#'
            {
                file_data.initial_loc = prev_motion;
                curr_dir = curr_dir.next();
            }
        }
    }

    return ans + 1;
}
fn possibility_finder(mut file_data: FileData) -> Vec<(usize, usize)> {
    let mut curr_dir = Direction::UP;
    let num_lines: usize = file_data.lines.len();
    let line_size: usize = file_data.lines[0].chars().count();
    let mut ans = Vec::new();
    while !done(&curr_dir, &file_data.initial_loc, &num_lines, &line_size) {
        if let Some(x) = file_data.lines[file_data.initial_loc.0]
            .as_mut()
            .chars()
            .nth(file_data.initial_loc.1)
        {
            if x != 'X' {
                ans.push((file_data.initial_loc.0, file_data.initial_loc.1));
                unsafe {
                    *file_data.lines[file_data.initial_loc.0]
                        .as_bytes_mut()
                        .index_mut(file_data.initial_loc.1) = b'X';
                }
            }
            let prev_motion = file_data.initial_loc.clone();
            curr_dir.to_motion(file_data.initial_loc.borrow_mut());
            if file_data.lines[file_data.initial_loc.0]
                .chars()
                .nth(file_data.initial_loc.1)
                .unwrap()
                == '#'
            {
                file_data.initial_loc = prev_motion;
                curr_dir = curr_dir.next();
            }
        }
    }
    ans.push((file_data.initial_loc.0, file_data.initial_loc.1));
    return ans;
}
fn cycle_finder(mut file_data: FileData) -> bool {
    let mut curr_dir = Direction::UP;
    let num_lines: usize = file_data.lines.len();
    let line_size: usize = file_data.lines[0].chars().count();
    while !done(&curr_dir, &file_data.initial_loc, &num_lines, &line_size) {
        if let Some(_) = file_data.lines[file_data.initial_loc.0]
            .as_mut()
            .chars()
            .nth(file_data.initial_loc.1)
        {
            let prev_motion = file_data.initial_loc.clone();
            curr_dir.to_motion(file_data.initial_loc.borrow_mut());
            let new_char = file_data.lines[file_data.initial_loc.0]
                .chars()
                .nth(file_data.initial_loc.1)
                .unwrap();
            if new_char == '#' {
                unsafe {
                    *file_data.lines[file_data.initial_loc.0]
                        .as_bytes_mut()
                        .index_mut(file_data.initial_loc.1) = match curr_dir {
                        Direction::LEFT => 1,
                        Direction::RIGHT => 2,
                        Direction::UP => 4,
                        Direction::DOWN => 8,
                    };
                }
                file_data.initial_loc = prev_motion;
                curr_dir = curr_dir.next();
            } else if new_char.is_ascii_control() {
                let dir_to_num = match curr_dir {
                    Direction::LEFT => 1,
                    Direction::RIGHT => 2,
                    Direction::UP => 4,
                    Direction::DOWN => 8,
                };
                unsafe {
                    let char_pt = file_data.lines[file_data.initial_loc.0]
                        .as_bytes_mut()
                        .index_mut(file_data.initial_loc.1);
                    if *char_pt & dir_to_num != 0 {
                        return true;
                    }
                    *char_pt |= match curr_dir {
                        Direction::LEFT => 1,
                        Direction::RIGHT => 2,
                        Direction::UP => 4,
                        Direction::DOWN => 8,
                    };
                }
                file_data.initial_loc = prev_motion;
                curr_dir = curr_dir.next();
            }
        }
    }
    return false;
}
pub fn part2(mut file_data: FileData) -> i32 {
    let possiblities = possibility_finder(file_data.clone());
    let mut ans = 0;
    let mut possibility_iter = possiblities.iter();
    possibility_iter.next();
    for breaker in possibility_iter {
        unsafe {
            *file_data.lines[breaker.0]
                .as_bytes_mut()
                .index_mut(breaker.1) = b'#';
        }
        if cycle_finder(file_data.clone()) {
            ans += 1;
            unsafe {
                *file_data.lines[breaker.0]
                    .as_bytes_mut()
                    .index_mut(breaker.1) = b'O';
            }
            unsafe {
                *file_data.lines[breaker.0]
                    .as_bytes_mut()
                    .index_mut(breaker.1) = b'.';
            }
        } else {
            unsafe {
                *file_data.lines[breaker.0]
                    .as_bytes_mut()
                    .index_mut(breaker.1) = b'.';
            }
        }
    }
    ans
}
