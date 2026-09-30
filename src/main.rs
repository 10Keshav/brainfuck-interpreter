use std::io::{self, Write};

fn build_bracket_map(program: &str) -> Vec<usize> {
    let bytes = program.as_bytes();
    let len = bytes.len();
    let mut map = vec![0usize; len];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..len {
        match bytes[i] {
            b'[' => {
                stack.push(i);
            }
            b']' => {
                let open = stack.pop().expect("unmatched");
                map[open] = i;
                map[i] = open;
            }
            _ => {}
        }
    }
    if !stack.is_empty() {
        panic!("unmatched [");
    }
    map
}

fn main() {
    // let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    print!("write: ");
    io::stdout().flush().unwrap();
    let mut program = String::new();
    io::stdin().read_line(&mut program).expect("helo");
    let program: &str = program.as_str();

    let mut tape = [0u8; 30000];
    let mut dp: usize = 0;
    let mut pc: usize = 0;

    let bracket_map = build_bracket_map(program);
    let bytes = program.as_bytes();

    let mut res = String::new();

    while pc < bytes.len() {
        match bytes[pc] {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => tape[dp] = tape[dp].wrapping_add(1),
            b'-' => tape[dp] = tape[dp].wrapping_sub(1),
            // b'.' => print!("{} ", tape[dp] as u8),
            b'.' => res.push(tape[dp] as char),

            b',' => {
                print!("\nval at cell: {dp} = ");
                io::stdout().flush().unwrap();
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("helo");
                let val: u16 = val.trim().parse().expect("not an int");
                let val = val as u8;
                tape[dp] = val;
            }

            b'[' if tape[dp] == 0 => pc = bracket_map[pc],

            b']' if tape[dp] != 0 => pc = bracket_map[pc],

            _ => {}
        }
        pc += 1;
    }
    println!("{res}");
}
