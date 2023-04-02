use std::io::{Read, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut program = input.trim().chars().filter(|&c| "+-><.,[]".contains(c)).collect::<String>();
    let mut stack = Vec::new();

    let mut tape = vec![0u8; 30000];
    let mut ptr = 0usize;

    let mut pc = 0usize;

    while pc < program.len() {
        match program.chars().nth(pc).unwrap() {
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                let mut buf = [0u8; 1];
                std::io::stdin().read_exact(&mut buf).unwrap();
                tape[ptr] = buf[0];
            },
            '[' => {
                if tape[ptr] == 0 {
                    let mut nest = 1usize;
                    while nest > 0 {
                        pc += 1;
                        match program.chars().nth(pc).unwrap() {
                            '[' => nest += 1,
                            ']' => nest -= 1,
                            _ => {}
                        }
                    }
                } else {
                    stack.push(pc);
                }
            },
            ']' => {
                if tape[ptr] != 0 {
                    pc = *stack.last().unwrap();
                } else {
                    stack.pop();
                }
            },
            _ => {}
        }
        pc += 1;
    }
}
