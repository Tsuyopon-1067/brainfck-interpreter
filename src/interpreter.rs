use crate::token::Token;

pub struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    pointer_stack: Vec<usize>,
    tokens: Vec<Token>,
    program_counter: usize,
    input: Vec<char>,
    input_pointer: usize,
}

impl Interpreter {
    pub fn new(content: String, input: String) -> Self {
        Interpreter {
            memory: vec![0; 30000],
            pointer: 0,
            pointer_stack: Vec::new(),
            tokens: content.chars().filter_map(Token::from_char).collect(),
            program_counter: 0,
            input: input.chars().collect(),
            input_pointer: 0,
        }
    }

    pub fn interpret(mut self) {
        while self.program_counter < self.tokens.len() {
            self.excute();
            self.program_counter += 1;
        }
        println!();
    }

    // pointer, program counter
    fn excute(&mut self) {
        match self.tokens[self.program_counter] {
            Token::IncrementPointer => {
                self.pointer += 1;
                if self.pointer >= self.memory.len() {
                    panic!("Pointer out of bounds");
                }
            }
            Token::DecrementPointer => {
                self.pointer -= 1;
            }
            Token::IncrementValue => {
                self.memory[self.pointer] += 1;
                if self.memory[self.pointer] == 0 {
                    panic!("Memory overflow");
                }
            }
            Token::DecrementValue => {
                self.memory[self.pointer] -= 1;
                if self.memory[self.pointer] == 255 {
                    panic!("Memory underflow");
                }
            }
            Token::Output => {
                let current_char = self.memory[self.pointer] as char;
                print!("{}", current_char);
            }
            Token::Input => {
                if self.input_pointer >= self.input.len() {
                    panic!("Input underflow");
                }
                self.memory[self.pointer] = self.input[self.input_pointer] as u8;
            }
            Token::JumpForward => {
                self.pointer_stack.push(self.program_counter);
            }
            Token::JumpBackward => {
                if self.memory[self.pointer] != 0 {
                    self.program_counter = *self
                        .pointer_stack
                        .last()
                        .expect("Unmatched closing bracket");
                } else {
                    self.pointer_stack.pop();
                }
            }
        }
    }
}
