use core::panic;
use std::{
    env::{self, Args},
    fs::{self, File},
    io::{BufRead, BufReader},
};
#[derive(Debug)]
enum ParserState {
    LetterM,
    LetterU,
    LetterL,
    LetterD,
    LetterO,
    LetterN,
    APOSTROPHE,
    LetterT,
    MulOpenParen,
    Number1_1,
    Number1_2,
    Number1_3,
    COMMA,
    Number2_1,
    Number2_2,
    Number2_3,
    MulCloseParen,
    DISABLED,
    DoOpenParen,
    DontOpenParen,
    DoClosedParen,
    DontClosedParen,
    JUNK,
}
struct Settings {
    pub part: bool,
    pub file_path: Box<str>,
}
impl Settings {
    fn parse_args(mut args: Args) -> Settings {
        if args.len() == 1 {
            Settings {
                part: false,
                file_path: Box::from("/home/rgarber11/advent_of_code/2024/day03/input"),
            }
        } else if args.len() == 2 {
            Settings {
                part: false,
                file_path: Box::from(args.nth(1).expect("This exists")),
            }
        } else if args.len() == 3 {
            Settings {
                part: match args.nth(1).expect("This exists").as_str() {
                    "1" => false,
                    "2" => true,
                    _ => panic!("Only parts 1 or 2 are options"),
                },
                file_path: Box::from(args.nth(2).expect("This exists")),
            }
        } else {
            panic!("Improper arguments. Try again.")
        }
    }
}
fn part1(settings: &Settings) -> Option<i64> {
    let file = File::open(settings.file_path.as_ref()).ok()?;
    let mut reader = BufReader::new(file);
    let mut parser_state = ParserState::JUNK;
    let mut number_buffer: [char; 4] = ['0'; 4];
    let ans = 0;
    let end = loop {
        let buffer = reader.fill_buf().unwrap();
        let size = buffer.len();
        if size == 0 {
            break;
        }
        for a in std::str::from_utf8(buffer).ok()?.chars() {
            parser_state = match parser_state {
                ParserState::LetterM => {
                    if a == 'u' {
                        ParserState::LetterU
                    } else {
                        ParserState::JUNK
                    }
                }
                ParserState::LetterU => {
                    if a == 'l' {
                        ParserState::LetterL
                    } else {
                        ParserState::JUNK
                    }
                }
                ParserState::LetterL => {
                    if a == '(' {
                        ParserState::MulOpenParen
                    } else {
                        ParserState::JUNK
                    }
                }
                ParserState::MulOpenParen => {
                    if a.is_digit(10) {
                        number_buffer[0] = a;
                        ParserState::Number1_1
                    } else {
                        ParserState::JUNK
                    }
                }
                ParserState::Number1_1 => todo!(),
                ParserState::Number1_2 => todo!(),
                ParserState::Number1_3 => todo!(),
                ParserState::COMMA => todo!(),
                ParserState::Number2_1 => todo!(),
                ParserState::Number2_2 => todo!(),
                ParserState::Number2_3 => todo!(),
                ParserState::MulCloseParen => todo!(),
                ParserState::JUNK => todo!(),
                _ => return None,
            }
        }
    };
    if end == None {
        return None;
    }
    return Some(ans);
}
fn part2(settings: &Settings) -> Option<i64> {}
fn main() {
    let settings = Settings::parse_args(env::args());
}
