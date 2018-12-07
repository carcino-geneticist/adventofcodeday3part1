use std::str::FromStr;
use std::io::{self, BufRead};
use std::collections::HashMap;

const BOARD_WIDTH: usize = 1000;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;
const ID: usize = 0;
const XPOS: usize = 1;
const YPOS: usize = 2;
const XSIZ: usize = 3;
const YSIZ: usize = 4;

#[derive(Clone, Copy)]
enum State {
    Unclaimed,
    Surviving(usize),
    Reclaimed
}

fn atoi(buffer: &str) -> usize {
    match usize::from_str(buffer) {
        Ok(res) => { res },
        Err(_) => { 0 }
    }
}

fn flush(rect: &mut [usize; 5], index: &mut usize, buffer: &mut String) {
    rect[*index] = atoi(buffer);
    buffer.clear();
    *index += 1;
}

fn main() {
    let mut surviving: HashMap<usize, bool> = HashMap::new();
    let mut state = vec![State::Unclaimed; BOARD_SIZE];
    let mut rect: [usize; 5] = [0; 5];
    let mut buffer = String::new();
    let mut index;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {

        index = 0;
        let mut printed = false;
        let line = line.unwrap();

        // load up rectangle
        for (i, c) in line.chars().enumerate() {
            if c == '#' { continue };
            if c >= '0' && c <= '9' {
                buffer.push(c);
            } else {
                if !c.is_whitespace() {
                    flush(&mut rect, &mut index, &mut buffer);
                }
            }
            if i == line.len() - 1 {
                flush(&mut rect, &mut index, &mut buffer);
            }
        }

        // draw it 
        for y in rect[YPOS]..rect[YPOS]+rect[YSIZ] {
            for x in rect[XPOS]..rect[XPOS]+rect[XSIZ] {
                let i = (y * BOARD_WIDTH) + x;
                if i >= state.len() { continue; }
                state[i] = match state[i] {
                    State::Unclaimed => {
                        surviving.entry(rect[ID]).or_insert(true);
                        State::Surviving(rect[ID])
                    },
                    State::Surviving(id) => {
                        if !printed { 
                            printed=true;
                            surviving.insert(id, false);
                            surviving.insert(rect[ID], false);
                            println!("{} reclaimed: {}", rect[ID], id);
                        }
                        State::Reclaimed
                    },
                    _ => State::Reclaimed
                }
            }
        }

    }

    // find it
    for (k, v) in &surviving {
        if *v { println!("PASS: {}", *k); }
    }
}
