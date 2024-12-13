use std::collections::{HashMap, HashSet};
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
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day05/input"),
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
    pub edges: Box<[(i32, i32)]>,
    pub manual_orders: Box<[Box<[i32]>]>,
}
impl FileData {
    pub fn read_file(file_path: &str) -> Option<FileData> {
        let file = File::open(file_path).ok()?;
        let mut line_reader = BufReader::new(file).lines();
        let edges: Box<[(i32, i32)]> = line_reader
            .by_ref()
            .map_while(|r| {
                let edge_str = r.unwrap();
                if edge_str.trim().is_empty() {
                    None
                } else {
                    let str_tup = edge_str
                        .split_once('|')
                        .expect("Edge Definitions should be split by |");
                    Some((
                        str_tup.0.parse().expect("First elem in edge should be num"),
                        str_tup
                            .1
                            .parse()
                            .expect("Second elem in edge should be num"),
                    ))
                }
            })
            .collect();
        let manual_orders: Box<[Box<[i32]>]> = line_reader
            .map(|r| {
                let order_str = r.unwrap();
                let str_iter = order_str.split(',');
                str_iter
                    .map(|s| s.parse().expect("Order members should be int"))
                    .collect::<Box<[i32]>>()
            })
            .collect();
        Some(FileData {
            edges,
            manual_orders,
        })
    }
}
