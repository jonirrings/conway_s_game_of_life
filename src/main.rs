extern crate conway_s_board;
extern crate regex;

use conway_s_board::{InitialBoard, Cell, CellState};
use std::fs::File;
use std::io::BufReader;

use regex::Regex;
use std::env;

fn load_initial_board(path: String) -> Option<InitialBoard> {
    let file = match File::open(path) {
        Ok(f) => f,
        _ => return None,
    };
    let mut file_buff = BufReader::new(&file);
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut gen: usize = 0;
    let cells: Vec<Cell>;
    for (idx, lineResult) in file_buff.lines().enumerate() {
        match lineResult {
            Ok(line) => {
                match idx {
                    0 => {
                        let re = Regex::new(r"^Generation \d+$").unwrap();
                        let caps_option = re.captures(line.as_str());
                        match caps_option {
                            Some(caps) => {
                                caps.get(0).map(|mat| {
                                    match mat {
                                        Some(gen_str) => {
                                            gen = usize::from_str(gen_str).unwrap_or(0);
                                        }
                                        _ => return None,
                                    }
                                });
                            }
                            _ => return None,
                        }
                    },
                    1=>{
                        let re = Regex::new(r"^\d+ \d+:$").unwrap();
                        let caps_option = re.captures(line.as_str());
                        match caps_option {
                            Some(caps)=>{
                                caps.get(0).map(|mat| {
                                    match mat {
                                        Some(rows_str) => {
                                            gen = usize::from_str(rows_str).unwrap_or(0);
                                        }
                                        _ => return None,
                                    }
                                });
                                caps.get(1).map(|mat| {
                                    match mat {
                                        Some(col_str) => {
                                            gen = usize::from_str(col_str).unwrap_or(0);
                                        }
                                        _ => return None,
                                    }
                                });
                            },
                            _=>return None,
                        }
                    },
                    _=>{
                        let re = Regex::new(r"[\.|*]").unwrap();
                        let caps_option = re.captures(line.as_str());
                        match caps_option {
                            Some(caps)=>{
                                for cap in caps.iter(){
                                    match cap {
                                        Some(c)=>{
                                            match c.as_ref() {
                                                "*"=>{
                                                    let cell = Cell{state:CellState::Alive};
                                                    cells.push(cell);
                                                },
                                                "."=>{
                                                    let cell = Cell{state:CellState::Dead};
                                                    cells.push(cell);
                                                },
                                                _=>return None,
                                            }
                                        },
                                        _=>return None,
                                    }
                                }
                            },
                            _=>return None,
                        }
                    }
                }
            }
            _ => return None,
        }
    };
    if rows == 0 || cols == 0 || gen == 0 {
        return None;
    }
    let board = Board::new(rows, cols, cells, gen, 200);
    Some(board)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1];
    let board = load_initial_board(filename);
    for b in board.iter(){
        println!(board);
    }
}

// todo: solve the type error
// todo: refactor for better organization
