use std::fs::File;
use std::vec;
use std::io::{prelude::*, BufReader};

const TAPE_SIZE: usize = 30000;

#[derive(Debug)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,

    IncrementValue,
    DecrementValue,

    OutputValue,
    InputValue,

    Begin(Option<usize>), // where to jump if zero
    End(usize), // where to jump if not zero

    Halt // added at the end of the buffer, prevents overflow if ] is the last instruction
}

#[derive(Debug)]
struct Intepreter {
    buffer: [u8; TAPE_SIZE],
    pointer: usize
}

impl Default for Intepreter {
    fn default() -> Self {
        Self {
            buffer: [0; TAPE_SIZE],
            pointer: TAPE_SIZE / 2
        }
    }
}

fn parse_code(code: &String) -> Result<Vec<Instruction>, &'static str> {
    let mut parsed_instructions: Vec<Instruction> = vec![];


    let mut stack: Vec<usize> = vec![]; // stack that keeps track of jump locations - [ ]

    for (index, operation) in code.chars().filter(|c| "><+-.,[]".contains(*c)).enumerate() {
        match operation {
            '>' => { parsed_instructions.push(Instruction::IncrementPointer); },
            '<' => { parsed_instructions.push(Instruction::DecrementPointer); },

            '+' => { parsed_instructions.push(Instruction::IncrementValue); },
            '-' => { parsed_instructions.push(Instruction::DecrementValue); },

            '.' => { parsed_instructions.push(Instruction::OutputValue); },
            ',' => { parsed_instructions.push(Instruction::InputValue); },

            '[' => {
                stack.push(index);
                parsed_instructions.push(Instruction::Begin(None));
            },

            ']' => {
                let previous_begin_index: usize = stack.pop().expect("Unmatched `]`, missing `[`");
                parsed_instructions[previous_begin_index] = Instruction::Begin(Some(index + 1));
                parsed_instructions.push(Instruction::End(previous_begin_index + 1));
            },

            _ => {}
        }

    }

    return Ok(parsed_instructions);
}

fn execute_code(parsed_code: Vec<Instruction>, interpreter: &mut Intepreter) {
    let mut instruction_index: usize = 0;
    let mut input_buffer: [u8; 1] = [0; 1];

    loop {

        match *parsed_code.get(instruction_index).unwrap() {
            Instruction::IncrementPointer => {
                if interpreter.pointer >= TAPE_SIZE { println!("Pointer out of bounds, overflow"); return; }
                interpreter.pointer += 1;

                instruction_index += 1;
            },
            Instruction::DecrementPointer => {
                if interpreter.pointer <= 0 { println!("Pointer out of bounds, underflow"); return; }
                interpreter.pointer -= 1;

                instruction_index += 1;
            },

            Instruction::IncrementValue => {
                interpreter.buffer[interpreter.pointer] = interpreter.buffer[interpreter.pointer].wrapping_add(1);
                instruction_index += 1;
            },
            Instruction::DecrementValue => {
                interpreter.buffer[interpreter.pointer] = interpreter.buffer[interpreter.pointer].wrapping_sub(1);
                instruction_index += 1;
            },

            Instruction::InputValue => {
                std::io::stdin().read_exact(&mut input_buffer).expect("Input error");
                interpreter.buffer[interpreter.pointer] = input_buffer[0];
            },
            Instruction::OutputValue => {
                print!("{}", interpreter.buffer[interpreter.pointer] as char);
                instruction_index += 1;
            },

            Instruction::Begin(jump_address) => {
                if interpreter.buffer[interpreter.pointer] == 0 {
                    instruction_index = jump_address.unwrap();
                } else {
                    instruction_index += 1;
                }
            },

            Instruction::End(jump_address) => {
                if interpreter.buffer[interpreter.pointer] != 0 {
                    instruction_index = jump_address;
                } else {
                    instruction_index += 1;
                }
            },

            Instruction::Halt => { /* print!("\n\nExecution ended\n"); */ break; },
        }
    }


}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath: &String = args.get(1).expect("Input filepath expected");

    let file: File = File::open(filepath).expect("File I/O error");
    let mut reader: BufReader<File> = BufReader::new(file);

    let mut file_content: String = String::new();
    reader.read_to_string(&mut file_content).expect("Error reading from file to a string");


    let mut interpreter: Intepreter = Intepreter::default();
    let parsed_instructions: Vec<Instruction> = match parse_code(&file_content) {
        Ok(mut instructions) => {
            instructions.push(Instruction::Halt);
            instructions
        },
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    execute_code(parsed_instructions, &mut interpreter);
}
