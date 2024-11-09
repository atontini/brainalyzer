use std::io::{self, Read};

use crate::config::is_debug;

pub struct BrainfuckParser {
    memory: Vec<u8>,
    pointer: usize,
}

impl BrainfuckParser {
    pub fn new() -> Self {
        BrainfuckParser {
            memory: vec![0; 30_000],
            pointer: 0,
        }
    }

    pub fn parse(&mut self, code: &str) {
        let code_chars: Vec<char> = code.chars().collect();
        let mut pc = 0;

        while pc < code_chars.len() {
            if is_debug() {
                println!("pc = {:?}", pc);
            }
            
            match code_chars[pc] {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                '-' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                '.' => print!("{}", self.memory[self.pointer] as char),
                ',' => self.memory[self.pointer] = io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .unwrap_or(0),
                '[' => {
                    if self.memory[self.pointer] == 0 {
                        let mut bracket_count = 1;
                        while bracket_count > 0 {
                            pc += 1;
                            if code_chars[pc] == '[' {
                                bracket_count += 1;
                            } else if code_chars[pc] == ']' {
                                bracket_count -= 1;
                            }
                        }
                    }
                }
                ']' => {
                    if self.memory[self.pointer] != 0 {
                        let mut bracket_count = 1;
                        while bracket_count > 0 {
                            pc -= 1;
                            if code_chars[pc] == ']' {
                                bracket_count += 1;
                            } else if code_chars[pc] == '[' {
                                bracket_count -= 1;
                            }
                        }
                    }
                }
                _ => {}
            }
            pc += 1;
        }
    }
}
