use std::{
    cmp::max,
    collections::{HashMap, HashSet},
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
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day08/input"),
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
    pub size: (usize, usize),
    pub antennae: HashMap<char, Vec<(usize, usize)>>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let file = File::open(file_path).ok()?;
        let lines = BufReader::new(file).lines().enumerate();
        let mut antennae: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let mut size = (0, 0);
        for (i, line_res) in lines {
            if let Some(line) = line_res.ok() {
                for (j, pt) in line.char_indices() {
                    if pt.is_alphabetic() || pt.is_digit(10) {
                        antennae.entry(pt).or_default().push((i, j));
                    }
                    size.0 = max(size.0, i + 1);
                    size.1 = max(size.1, j + 1);
                }
            } else {
                return None;
            }
        }
        return Some(FileData { size, antennae });
    }
}
pub fn part1(file_data: FileData) -> i64 {
    let mut ans = 0;
    let mut already_seen = HashSet::new();
    for (_, locs) in file_data.antennae {
        for i in 0..locs.len() {
            for j in (i + 1)..locs.len() {
                let loc1: (i64, i64) = (locs[i].0 as i64, locs[i].1 as i64);
                let loc2: (i64, i64) = (locs[j].0 as i64, locs[j].1 as i64);
                let diff = (loc2.0 - loc1.0, loc2.1 - loc1.1);
                let new_loc1 = (loc1.0 - diff.0, loc1.1 - diff.1);
                if new_loc1.0 >= 0
                    && new_loc1.0 < (file_data.size.0 as i64)
                    && new_loc1.1 >= 0
                    && new_loc1.1 < (file_data.size.1 as i64)
                    && !already_seen.contains(&new_loc1)
                {
                    ans += 1;
                    already_seen.insert(new_loc1);
                }
                let new_loc2 = (loc2.0 + diff.0, loc2.1 + diff.1);
                if new_loc2.0 >= 0
                    && new_loc2.0 < (file_data.size.0 as i64)
                    && new_loc2.1 >= 0
                    && new_loc2.1 < (file_data.size.1 as i64)
                    && !already_seen.contains(&new_loc2)
                {
                    ans += 1;
                    already_seen.insert(new_loc2);
                }
            }
        }
    }
    ans
}
fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}
pub fn part2(file_data: FileData) -> i64 {
    let mut ans = 0;
    let mut already_seen: HashSet<(i64, i64)> = HashSet::new();
    for (_, locs) in file_data.antennae {
        for i in 0..locs.len() {
            for j in (i + 1)..locs.len() {
                let loc1: (i64, i64) = (locs[i].0 as i64, locs[i].1 as i64);
                let loc2: (i64, i64) = (locs[j].0 as i64, locs[j].1 as i64);
                let diff = (loc2.0 - loc1.0, loc2.1 - loc1.1);
                let divisor = if diff.0 == 0 {
                    diff.1
                } else if diff.1 == 0 {
                    diff.0
                } else {
                    gcd(
                        if diff.0 < 0 { -diff.0 } else { diff.0 },
                        if diff.1 < 0 { -diff.1 } else { diff.1 },
                    )
                };
                let delta = (diff.0 / divisor, diff.1 / divisor);
                let mut new_loc1 = (loc1.0, loc1.1);
                while new_loc1.0 >= 0
                    && new_loc1.0 < (file_data.size.0 as i64)
                    && new_loc1.1 >= 0
                    && new_loc1.1 < (file_data.size.1 as i64)
                {
                    if !already_seen.contains(&new_loc1) {
                        ans += 1;
                        already_seen.insert(new_loc1.clone());
                    }
                    new_loc1 = (new_loc1.0 - delta.0, new_loc1.1 - delta.1);
                }
                new_loc1 = (loc1.0, loc1.1);
                while new_loc1.0 >= 0
                    && new_loc1.0 < (file_data.size.0 as i64)
                    && new_loc1.1 >= 0
                    && new_loc1.1 < (file_data.size.1 as i64)
                {
                    if !already_seen.contains(&new_loc1) {
                        ans += 1;
                        already_seen.insert(new_loc1.clone());
                    }
                    new_loc1 = (new_loc1.0 + delta.0, new_loc1.1 + delta.1);
                }
            }
        }
    }
    ans
}
