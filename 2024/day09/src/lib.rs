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
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day09/input"),
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
    files: Box<[i8]>,
    holes: Box<[i8]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let mut file_str = String::new();
        let _ = File::open(file_path)
            .ok()?
            .read_to_string(&mut file_str)
            .ok()?;
        let (block_chars, hole_chars): (Vec<(usize, char)>, Vec<(usize, char)>) = file_str
            .char_indices()
            .filter(|tup| tup.1.is_digit(10))
            .partition(|tup| tup.0 % 2 == 0);
        Some(FileData {
            files: block_chars
                .into_iter()
                .map(|tup| tup.1.to_digit(10).unwrap() as i8)
                .collect(),
            holes: hole_chars
                .into_iter()
                .map(|tup| tup.1.to_digit(10).unwrap() as i8)
                .collect(),
        })
    }
}
pub fn part1(mut file_data: FileData) -> i64 {
    let mut ans = 0;
    let mut forward_block = 0;
    let mut backward_block = file_data.files.len() - 1;
    let mut curr_hole = 0;
    let mut curr_idx = 0;
    let mut forward_state = true;
    while forward_block != backward_block || file_data.files[forward_block] != 0 {
        if forward_state {
            for _ in 0..file_data.files[forward_block] {
                ans += curr_idx * forward_block;
                curr_idx += 1;
            }
            file_data.files[forward_block] = 0;
            forward_block += 1;
            forward_state = false;
        } else {
            while file_data.holes[curr_hole] >= file_data.files[backward_block]
                && backward_block != 0
            {
                for _ in 0..file_data.files[backward_block] {
                    ans += curr_idx * backward_block;
                    curr_idx += 1;
                }
                file_data.holes[curr_hole] -= file_data.files[backward_block];
                file_data.files[backward_block] = 0;
                backward_block -= 1;
            }
            if backward_block == 0 {
                break;
            }
            for _ in 0..file_data.holes[curr_hole] {
                ans += curr_idx * backward_block;
                curr_idx += 1;
            }
            file_data.files[backward_block] -= file_data.holes[curr_hole];
            file_data.holes[curr_hole] = 0;
            curr_hole += 1;
            forward_state = true;
        }
    }
    ans as i64
}
pub fn part2(file_data: FileData) -> i64 {
    let total_len = file_data.files.len() + file_data.holes.len();
    let mut ranges: Vec<(i64, i8)> = Vec::with_capacity(total_len);
    ranges.push((0, file_data.files[0]));
    for i in 1..total_len {
        if i % 2 == 0 {
            ranges.push((
                ranges[i - 1].0 + ranges[i - 1].1 as i64,
                file_data.files[i / 2],
            ));
        } else {
            ranges.push((
                ranges[i - 1].0 + ranges[i - 1].1 as i64,
                file_data.holes[(i - 1) / 2],
            ));
        }
    }
    let mut ans = 0;
    for i in (0..file_data.files.len()).rev() {
        let size_needed = file_data.files[i];
        let mut found_hole = false;
        for j in (1..(i * 2)).step_by(2) {
            if ranges[j].1 < size_needed {
                continue;
            }
            found_hole = true;
            for idx in ranges[j].0..(ranges[j].0 + size_needed as i64) {
                ans += idx * i as i64;
            }
            ranges[j].0 += size_needed as i64;
            ranges[j].1 -= size_needed;
            break;
        }
        if !found_hole {
            for idx in ranges[i * 2].0..(ranges[i * 2].0 + size_needed as i64) {
                ans += idx * i as i64;
            }
        }
    }
    ans
}
