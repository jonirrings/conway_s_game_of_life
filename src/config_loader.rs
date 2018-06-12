use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use regex::Regex;

use board::Board;

pub fn load_config(path: &String) -> Option<Board> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        _ => return None,
    };
    let mut config_buffer = String::new();
    file.read_to_string(&mut config_buffer).unwrap();
    parse_config(config_buffer.as_str())
}

pub fn parse_config(config: &str) -> Option<Board> {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut gen: usize = 0;
    let mut cells: Vec<u8> = Vec::new();
    for (idx, line) in config.lines().enumerate() {
        match idx {
            0 => {
                let re = Regex::new(r"^Generation (\d+):$").unwrap();
                let caps_option = re.captures(line);
                match caps_option {
                    Some(caps) => {
                        caps.get(1).map(|mat| {
                            gen = usize::from_str(mat.as_str()).unwrap_or(0);
                        });
                    }
                    _ => return None,
                }
            }
            1 => {
                let re = Regex::new(r"^(\d+) (\d+)$").unwrap();
                let caps_option = re.captures(line);
                match caps_option {
                    Some(caps) => {
                        caps.get(1).map(|mat| {
                            rows = usize::from_str(mat.as_str()).unwrap_or(0);
                        });
                        caps.get(2).map(|mat| {
                            cols = usize::from_str(mat.as_str()).unwrap_or(0);
                        });
                    }
                    _ => return None,
                }
            }
            _ => {
                let line_bytes = line.as_bytes();
                for ch in line_bytes.iter() {
                    match *ch {
                        b'*' => cells.push(1),
                        b'.' => cells.push(0),
                        _ => return None,
                    }
                }
            }
        }
    }
    if rows == 0 || cols == 0 || gen == 0 || cells.len() == 0 {
        return None;
    }
    if rows * cols != cells.len() {
        return None;
    }
    let board = Board::new(rows, cols, cells, gen);
    Some(board)
}

// todo: add unit tests

#[cfg(test)]
mod tests {
    #[test]
    fn config_load() {
        assert_eq!(4, 2 + 2);
    }
}
