use std::{env::Args, fs::File, io::Read};
pub struct Settings {
    pub is_part_two: bool,
    pub file_path: Box<str>,
}
impl Settings {
    pub fn parse_args(mut args: Args) -> Option<Settings> {
        match args.len() {
            1 => Some(Settings {
                is_part_two: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day04/input"),
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
enum MatcherStates {
    JUNK,
    FORWARD,
    BACK,
}
pub fn read_lines(file_path: &str) -> Option<Box<[Box<str>]>> {
    let mut buffer = String::new();
    File::open(file_path)
        .ok()?
        .read_to_string(&mut buffer)
        .ok()?;
    let lines: Box<[Box<str>]> = buffer.lines().map(|i| Box::from(i)).collect();
    let len = lines[0].chars().count();
    if !lines.iter().map(|e| e.chars().count()).all(|e| e == len) {
        return None;
    }
    return Some(lines);
}
struct MatcherResult(i64, char, MatcherStates);

fn matcher(acc: MatcherResult, curr: char) -> MatcherResult {
    match acc.2 {
        MatcherStates::JUNK => match curr {
            'X' => MatcherResult(acc.0, 'X', MatcherStates::FORWARD),
            'S' => MatcherResult(acc.0, 'S', MatcherStates::BACK),
            _ => MatcherResult(acc.0, curr, MatcherStates::JUNK),
        },
        MatcherStates::FORWARD => match curr {
            'X' => MatcherResult(acc.0, 'X', MatcherStates::FORWARD),
            'M' => {
                if acc.1 == 'X' {
                    MatcherResult(acc.0, 'M', MatcherStates::FORWARD)
                } else {
                    MatcherResult(acc.0, 'M', MatcherStates::JUNK)
                }
            }
            'A' => {
                if acc.1 == 'M' {
                    MatcherResult(acc.0, 'A', MatcherStates::FORWARD)
                } else {
                    MatcherResult(acc.0, 'A', MatcherStates::JUNK)
                }
            }
            'S' => {
                if acc.1 == 'A' {
                    MatcherResult(acc.0 + 1, 'S', MatcherStates::BACK)
                } else {
                    MatcherResult(acc.0, 'S', MatcherStates::BACK)
                }
            }
            _ => MatcherResult(acc.0, curr, MatcherStates::JUNK),
        },
        MatcherStates::BACK => match curr {
            'S' => MatcherResult(acc.0, 'S', MatcherStates::BACK),
            'A' => {
                if acc.1 == 'S' {
                    MatcherResult(acc.0, 'A', MatcherStates::BACK)
                } else {
                    MatcherResult(acc.0, 'A', MatcherStates::JUNK)
                }
            }
            'M' => {
                if acc.1 == 'A' {
                    MatcherResult(acc.0, 'M', MatcherStates::BACK)
                } else {
                    MatcherResult(acc.0, 'M', MatcherStates::JUNK)
                }
            }
            'X' => {
                if acc.1 == 'M' {
                    MatcherResult(acc.0 + 1, 'X', MatcherStates::FORWARD)
                } else {
                    MatcherResult(acc.0, 'X', MatcherStates::FORWARD)
                }
            }
            _ => MatcherResult(acc.0, curr, MatcherStates::JUNK),
        },
    }
}
fn cross_checker(loc: (usize, usize), arr: &Box<[Box<str>]>) -> bool {
    let mut upper_str_iter = arr[loc.0 - 1].chars();
    let mut lower_str_iter = arr[loc.0 + 1].chars();
    let up_left = upper_str_iter
        .nth(loc.1 - 1)
        .expect("Failure of bounds-checking");
    let up_right = upper_str_iter.nth(1).expect("Failure of bounds-checking");
    let down_left = lower_str_iter
        .nth(loc.1 - 1)
        .expect("Failure of bounds-checking");
    let down_right = lower_str_iter.nth(1).expect("Failure of bounds-checking");
    let left_mas = (up_left == 'M' && down_right == 'S') || (up_left == 'S' && down_right == 'M');
    let right_mas = (up_right == 'M' && down_left == 'S') || (up_right == 'S' && down_left == 'M');
    return left_mas && right_mas;
}
pub fn part1(lines: Box<[Box<str>]>) -> Result<i64, &'static str> {
    let mut ans = 0;
    for line in &lines {
        // Rows
        ans += line
            .chars()
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    let strlen = lines[0].chars().count();
    for i in 0..strlen {
        // Columns
        ans += (0..lines.len())
            .map(|n| unsafe { lines[n].chars().nth(i).unwrap_unchecked() })
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    for i in 0..(strlen - 3) {
        // Rightward diagonals intersecting with top row.
        ans += (i..strlen)
            .zip(0..lines.len())
            .map(|tup| unsafe { lines[tup.1].chars().nth(tup.0).unwrap_unchecked() })
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    for i in 1..(lines.len() - 3) {
        // Rightward diagonals intersecting with first column
        ans += (0..strlen)
            .zip(i..lines.len())
            .map(|tup| unsafe { lines[tup.1].chars().nth(tup.0).unwrap_unchecked() })
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    for i in 3..strlen {
        // Leftward diagonals intersecting the top row.
        ans += (0..=i)
            .rev()
            .zip(0..lines.len())
            .map(|tup| unsafe { lines[tup.1].chars().nth(tup.0).unwrap_unchecked() })
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    for i in 1..(lines.len() - 3) {
        // Leftward diagonals intersecting with last column.
        ans += (0..strlen)
            .rev()
            .zip(i..lines.len())
            .map(|tup| unsafe { lines[tup.1].chars().nth(tup.0).unwrap_unchecked() })
            .fold(MatcherResult(0, '_', MatcherStates::JUNK), matcher)
            .0;
    }
    Ok(ans)
}
pub fn part2(lines: Box<[Box<str>]>) -> Result<i64, &'static str> {
    let mut ans = 0;
    let str_len = lines[0].chars().count();
    for i in 1..(lines.len() - 1) {
        let mut char_iter = lines[i].char_indices();
        char_iter.next();
        for (j, member) in char_iter {
            if j == str_len - 1 {
                break;
            }
            if member == 'A' && cross_checker((i, j), &lines) {
                ans += 1;
            }
        }
    }
    return Ok(ans);
}
