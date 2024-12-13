use std::{
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
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day07/input"),
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
pub struct LineData {
    pub test_val: i64,
    pub vals: Box<[i64]>,
}
pub struct FileData {
    pub lines: Box<[LineData]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let files = File::open(file_path).ok()?;
        let lines = BufReader::new(files).lines();
        let ans_inner: Box<[LineData]> = lines
            .into_iter()
            .map(|line| {
                let line_str = line.expect("Cannot handle line not existing.");
                let tup = line_str.split_once(':').expect("Line must have colon");
                let vals: Box<[i64]> = tup
                    .1
                    .split_whitespace()
                    .map(|num_str| num_str.parse::<i64>().expect("Must be num"))
                    .collect();
                LineData {
                    test_val: tup.0.parse::<i64>().expect("First value must fit in i64"),
                    vals,
                }
            })
            .collect();
        return Some(FileData { lines: ans_inner });
    }
}
fn can_make_equal_part1(vals: &[i64], target: i64) -> Option<bool> {
    if vals.len() - 1
        > (usize::BITS - 1)
            .try_into()
            .expect("The size of usize fits in a usize, come on")
    {
        return None;
    }
    if vals.len() == 1 {
        return Some(vals[0] == target);
    }
    let end_goal: u64 = 1 << vals.len() - 1;
    for i in 0..end_goal {
        let ans = (0..(vals.len() - 1)).fold(vals[0], |acc, j| match (i >> j) & 1 {
            0 => acc + vals[j + 1],
            _ => acc * vals[j + 1],
        });
        if ans == target {
            return Some(true);
        }
    }
    Some(false)
}
fn can_make_equal_part2(vals: &[i64], target: i64) -> Option<bool> {
    if vals.len() - 1
        > ((usize::BITS - 1) / 2)
            .try_into()
            .expect("The size of usize fits in a usize, come on")
    {
        return None;
    }
    if vals.len() == 1 {
        return Some(vals[0] == target);
    }
    let end_goal: u64 = 1 << ((vals.len() - 1) * 2);
    for i in 0..end_goal {
        let ans = (0..(vals.len() - 1)).fold(vals[0], |acc, j| match (i >> (2 * j)) & 3 {
            0 => acc + vals[j + 1],
            1 => acc * vals[j + 1],
            _ => {
                let mut temp_ans = acc;
                let num_digits = ((vals[j + 1] as f64).log10().floor() as i64) + 1;
                for _ in 0..num_digits {
                    temp_ans *= 10;
                }
                temp_ans + vals[j + 1]
            }
        });
        if ans == target {
            return Some(true);
        }
    }
    Some(false)
}
pub fn part1(file_data: FileData) -> i64 {
    let mut ans: i64 = 0;
    for line in file_data.lines {
        if can_make_equal_part1(line.vals.as_ref(), line.test_val)
            .expect("Hopefully there will be fewer than 64 in a line")
        {
            ans += line.test_val;
        }
    }
    ans
}
pub fn part2(file_data: FileData) -> i64 {
    let mut ans: i64 = 0;
    for line in file_data.lines {
        if can_make_equal_part2(line.vals.as_ref(), line.test_val)
            .expect("Hopefully there will be fewer than 32 in a line")
        {
            ans += line.test_val;
        }
    }
    ans
}
