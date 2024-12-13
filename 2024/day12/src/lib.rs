use std::collections::VecDeque;
use std::env::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Settings {
    pub is_part_two: bool,
    pub file_path: Box<str>,
}
impl Settings {
    pub fn parse_args(mut args: Args) -> Option<Settings> {
        match args.len() {
            1 => Some(Settings {
                is_part_two: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day12/input"),
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
    pub lines: Box<[Box<[u8]>]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let file = File::open(file_path).ok()?;
        let line_iter = BufReader::new(file).lines();
        let string_lines: Box<[String]> = line_iter
            .into_iter()
            .map(|l| {
                let res = l.expect("Can't deal with no string");
                res
            })
            .collect();
        if string_lines.iter().any(|line| !line.is_ascii()) {
            return None;
        }
        let lines: Box<[Box<[u8]>]> = string_lines
            .iter()
            .map(|line| line.chars().map(|c| c as u8).collect())
            .collect();
        Some(FileData { lines })
    }
}
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn has_external_side(
    pos: (usize, usize),
    direction: Direction,
    field: &[Box<[u8]>],
    crop: u8,
) -> bool {
    match direction {
        Direction::UP => pos.0 == 0 || field[pos.0 - 1][pos.1] != crop,
        Direction::DOWN => pos.0 == field.len() - 1 || field[pos.0 + 1][pos.1] != crop,
        Direction::LEFT => pos.1 == 0 || field[pos.0][pos.1 - 1] != crop,
        Direction::RIGHT => pos.1 == field[0].len() - 1 || field[pos.0][pos.1 + 1] != crop,
    }
}
fn area_perimeter_calculator(
    pos: (usize, usize),
    field: &[Box<[u8]>],
    visited: &mut [Box<[bool]>],
) -> (i64, i64) {
    let mut queue = VecDeque::from([pos]);
    let mut area = 0;
    let mut perimeter = 0;
    let crop = field[pos.0][pos.1];
    visited[pos.0][pos.1] = true;
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        area += 1;
        if has_external_side(curr, Direction::UP, field, crop) {
            perimeter += 1;
        } else if !visited[curr.0 - 1][curr.1] {
            visited[curr.0 - 1][curr.1] = true;
            queue.push_back((curr.0 - 1, curr.1));
        }
        if has_external_side(curr, Direction::DOWN, field, crop) {
            perimeter += 1;
        } else if !visited[curr.0 + 1][curr.1] {
            visited[curr.0 + 1][curr.1] = true;
            queue.push_back((curr.0 + 1, curr.1));
        }
        if has_external_side(curr, Direction::LEFT, field, crop) {
            perimeter += 1;
        } else if !visited[curr.0][curr.1 - 1] {
            visited[curr.0][curr.1 - 1] = true;
            queue.push_back((curr.0, curr.1 - 1));
        }
        if has_external_side(curr, Direction::RIGHT, field, crop) {
            perimeter += 1;
        } else if !visited[curr.0][curr.1 + 1] {
            visited[curr.0][curr.1 + 1] = true;
            queue.push_back((curr.0, curr.1 + 1));
        }
    }
    (area, perimeter)
}
fn not_visited_side(pos: (usize, usize), direction: Direction, visited: &mut [Box<[u8]>]) -> bool {
    match direction {
        Direction::UP => visited[pos.0][pos.1] & 2 == 0,
        Direction::DOWN => visited[pos.0][pos.1] & 4 == 0,
        Direction::LEFT => visited[pos.0][pos.1] & 8 == 0,
        Direction::RIGHT => visited[pos.0][pos.1] & 16 == 0,
    }
}
fn area_sides_calculator(
    pos: (usize, usize),
    field: &[Box<[u8]>],
    visited: &mut [Box<[u8]>],
) -> (i64, i64) {
    let mut queue = VecDeque::from([pos]);
    let mut area = 1;
    let mut num_sides = 0;
    let crop = field[pos.0][pos.1];
    let line_len = field[0].len();
    visited[pos.0][pos.1] = 1;
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        if has_external_side(curr, Direction::UP, field, crop) {
            if not_visited_side(curr, Direction::UP, visited) {
                num_sides += 1;
                for j in (0..curr.1).rev() {
                    if field[curr.0][j] != crop
                        || !has_external_side((curr.0, j), Direction::UP, field, crop)
                    {
                        break;
                    }
                    visited[curr.0][j] |= 2;
                }
                for j in curr.1..line_len {
                    if field[curr.0][j] != crop
                        || !has_external_side((curr.0, j), Direction::UP, field, crop)
                    {
                        break;
                    }
                    visited[curr.0][j] |= 2;
                }
            }
        } else if visited[curr.0 - 1][curr.1] & 1 == 0 {
            area += 1;
            visited[curr.0 - 1][curr.1] |= 1;
            queue.push_back((curr.0 - 1, curr.1));
        }
        if has_external_side(curr, Direction::DOWN, field, crop) {
            if not_visited_side(curr, Direction::DOWN, visited) {
                num_sides += 1;
                for j in (0..curr.1).rev() {
                    if field[curr.0][j] != crop
                        || !has_external_side((curr.0, j), Direction::DOWN, field, crop)
                    {
                        break;
                    }
                    visited[curr.0][j] |= 4;
                }
                for j in curr.1..line_len {
                    if field[curr.0][j] != crop
                        || !has_external_side((curr.0, j), Direction::DOWN, field, crop)
                    {
                        break;
                    }
                    visited[curr.0][j] |= 4;
                }
            }
        } else if visited[curr.0 + 1][curr.1] & 1 == 0 {
            area += 1;
            visited[curr.0 + 1][curr.1] |= 1;
            queue.push_back((curr.0 + 1, curr.1));
        }
        if has_external_side(curr, Direction::LEFT, field, crop) {
            if not_visited_side(curr, Direction::LEFT, visited) {
                num_sides += 1;
                for i in (0..curr.0).rev() {
                    if field[i][curr.1] != crop
                        || !has_external_side((i, curr.1), Direction::LEFT, field, crop)
                    {
                        break;
                    }
                    visited[i][curr.1] |= 8;
                }
                for i in curr.0..field.len() {
                    if field[i][curr.1] != crop
                        || !has_external_side((i, curr.1), Direction::LEFT, field, crop)
                    {
                        break;
                    }
                    visited[i][curr.1] |= 8;
                }
            }
        } else if visited[curr.0][curr.1 - 1] & 1 == 0 {
            area += 1;
            visited[curr.0][curr.1 - 1] |= 1;
            queue.push_back((curr.0, curr.1 - 1));
        }
        if has_external_side(curr, Direction::RIGHT, field, crop) {
            if not_visited_side(curr, Direction::RIGHT, visited) {
                num_sides += 1;
                for i in (0..curr.0).rev() {
                    if field[i][curr.1] != crop
                        || !has_external_side((i, curr.1), Direction::RIGHT, field, crop)
                    {
                        break;
                    }
                    visited[i][curr.1] |= 16;
                }
                for i in curr.0..field.len() {
                    if field[i][curr.1] != crop
                        || !has_external_side((i, curr.1), Direction::RIGHT, field, crop)
                    {
                        break;
                    }
                    visited[i][curr.1] |= 16;
                }
            }
        } else if visited[curr.0][curr.1 + 1] & 1 == 0 {
            area += 1;
            visited[curr.0][curr.1 + 1] |= 1;
            queue.push_back((curr.0, curr.1 + 1));
        }
    }
    (area, num_sides)
}
pub fn part1(file_data: FileData) -> i64 {
    let line_len = file_data.lines[0].len();
    let mut visited: Box<[Box<[bool]>]> = (0..file_data.lines.len())
        .map(|_| std::iter::repeat_n(false, line_len).collect())
        .collect();
    let mut ans = 0;
    for i in 0..file_data.lines.len() {
        for j in 0..line_len {
            if !visited[i][j] {
                let (area, perimeter) =
                    area_perimeter_calculator((i, j), file_data.lines.as_ref(), visited.as_mut());
                ans += area * perimeter;
            }
        }
    }
    ans
}
pub fn part2(file_data: FileData) -> i64 {
    let line_len = file_data.lines[0].len();
    let mut visited: Box<[Box<[u8]>]> = (0..file_data.lines.len())
        .map(|_| std::iter::repeat_n(0, line_len).collect())
        .collect();
    let mut ans = 0;
    for i in 0..file_data.lines.len() {
        for j in 0..line_len {
            if visited[i][j] == 0 {
                let (area, num_sides) =
                    area_sides_calculator((i, j), file_data.lines.as_ref(), visited.as_mut());
                ans += area * num_sides;
            }
        }
    }
    ans
}
