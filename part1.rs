use std::str::FromStr;
use std::io::{self, BufRead};

const BOARD_WIDTH: usize = 1000;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;
const XPOS: usize = 0;
const YPOS: usize = 1;
const XSIZ: usize = 2;
const YSIZ: usize = 3;

fn atoi(buffer: &str) -> usize {
    match usize::from_str(buffer) {
        Ok(res) => { res },
        Err(_) => { 0 }
    }
}

fn main() {
    let mut state = vec![0; BOARD_SIZE];
    let mut buffer = String::new();
    let mut rect: [usize; 4] = [0; 4];
    let mut ignore;
    let mut index;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        index = 0;
        ignore = true;
        let line = line.unwrap();

        // load up rectangle
        for (i, c) in line.chars().enumerate() {
            if c >= '0' && c <= '9' {
                if !ignore { buffer.push(c); }
            } else {
                if c == '@' { ignore = false; continue; }
                if c != '#' && !c.is_whitespace() {
                    rect[index] = atoi(&buffer);
                    buffer.clear();
                    index += 1;
                }
            }
            if i == line.len() - 1 {
                rect[index] = atoi(&buffer);
                buffer.clear();
                index += 1;
            }
        }

        // draw it 
        for y in rect[YPOS]..rect[YPOS]+rect[YSIZ] {
            for x in rect[XPOS]..rect[XPOS]+rect[XSIZ] {
                let i = (y * BOARD_WIDTH) + x;
                if i >= state.len() { continue; }
                state[i] += 1;
            }
        }
    }

    // count how many >1's we have
    let mut result = 0;
    for x in &state {
        if *x > 1 { result += 1; }
    }
    println!("{}", result);
}
