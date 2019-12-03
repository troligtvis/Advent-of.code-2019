use crate::utils;

pub fn solve() {
    let lines = utils::lines_from_file("input_2.1.txt");
    let input: Vec<i32> = lines[0]
        .split(",")
        .map(|s|
            s.to_string()
                .parse()
                .unwrap()
        )
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut list = input.clone();
            if let Some(code) = int_code(&mut list, noun, verb) {
                if noun == 12 && verb == 2 {
                    println!("Day2/Part 1: {}", list[0]);
                }

                if code == 19690720 {
                    let result = 100 * noun + verb;
                    println!("Day2/Part 2: {}", result);
                }
            }
        }
    }
}

fn int_code(list: &mut Vec<i32>, noun: i32, verb: i32) -> Option<i32> {
    // Update address 1 & 2 with current noun & verb
    list[1] = noun;
    list[2] = verb;

    let max = list.len();
    let mut i = 0;
    let mut v = list[0];

    loop {
        // Calculate indices depending on current index
        let j = list[(i + 1) % max] as usize;
        let k = list[(i + 2) % max] as usize;
        let l = list[(i + 3) % max] as usize;

        let op_code = OpCode::from_i32(
            v,
            list[j % max],
            list[k % max]
        );

        if let Some(c) = op_code {
            if c == OpCode::Halt {
                return Some(list[0])
            }

            // Calculate output with correct Opcode
            list[l % max] = c.calculate();

            // Step forward 4 positions
            i = (i + 4) % max;
        } else {
            // Step forward
            i = (i + 1) % max;
        }

        v = list[i];
    }
}

#[derive(PartialEq)]
enum OpCode {
    Add(i32, i32),
    Mul(i32, i32),
    Halt,
}

impl OpCode {
    fn from_i32(value: i32, noun: i32, verb: i32) -> Option<OpCode> {
        match value {
            1 => Some(OpCode::Add(noun, verb)),
            2 => Some(OpCode::Mul(noun, verb)),
            99 => Some(OpCode::Halt),
            _ => None,
        }
    }

    fn calculate(&self) -> i32 {
        match self {
            OpCode::Add(noun, verb) => noun + verb,
            OpCode::Mul(noun, verb) => noun * verb,
            _ => 0,
        }
    }
}
