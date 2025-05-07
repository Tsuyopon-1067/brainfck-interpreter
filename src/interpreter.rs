use crate::token::Token;

pub struct Interpreter {
    debug: bool,
    memory: Vec<u8>,
    pointer: usize,
    pointer_stack: Vec<usize>,
    tokens: Vec<Token>,
    program_counter: usize,
    input: Vec<char>,
    input_pointer: usize,
}

impl Interpreter {
    pub fn new(content: String, input: String, debug: bool) -> Self {
        Interpreter {
            debug: debug,
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

    fn excute(&mut self) {
        match self.tokens[self.program_counter] {
            Token::IncrementPointer => {
                self.pointer = self.increment_pointer(self.pointer);
            }
            Token::DecrementPointer => {
                self.pointer = self.decrement_pointer(self.pointer);
            }
            Token::IncrementValue => {
                self.memory[self.pointer] = self.increment_value(self.memory[self.pointer]);
            }
            Token::DecrementValue => {
                self.memory[self.pointer] = self.decrement_value(self.memory[self.pointer]);
            }
            Token::Output => {
                let current_char = self.memory[self.pointer];
                if self.debug {
                    print!("{} ", current_char);
                } else {
                    print!("{}", current_char as char);
                }
            }
            Token::Input => {
                if self.input_pointer >= self.input.len() {
                    panic!("{}", self.create_error_message("Input out of bounds"));
                }
                self.memory[self.pointer] = self.input[self.input_pointer] as u8;
            }
            Token::JumpForward => {
                self.pointer_stack.push(self.program_counter);
                if self.memory[self.pointer] == 0 {
                    let mut depth = 0;
                    while self.tokens[self.program_counter] != Token::JumpBackward && depth == 0 {
                        self.program_counter += 1;
                        if self.program_counter >= self.tokens.len() {
                            panic!("{}", self.create_error_message("Unmatched opening bracket"));
                        }
                        if self.tokens[self.program_counter] == Token::JumpForward {
                            depth += 1;
                        } else if self.tokens[self.program_counter] == Token::JumpBackward {
                            depth -= 1;
                        }
                    }
                    self.pointer_stack.pop();
                }
            }
            Token::JumpBackward => {
                if self.memory[self.pointer] == 0 {
                    self.pointer_stack.pop();
                } else {
                    self.program_counter = *self
                        .pointer_stack
                        .last()
                        .expect(&self.create_error_message("Unmatched closing bracket"));
                }
            }
        }
    }

    fn create_error_message(&self, message: &str) -> String {
        format!(
            "Error at program counter {}: {}",
            self.program_counter, message
        )
    }

    fn increment_value(&self, current_value: u8) -> u8 {
        if current_value == 255 {
            0
        } else {
            current_value + 1
        }
    }

    fn decrement_value(&self, current_value: u8) -> u8 {
        if current_value == 0 {
            255
        } else {
            current_value - 1
        }
    }

    fn increment_pointer(&self, current_value: usize) -> usize {
        if current_value == self.memory.len() - 1 {
            0
        } else {
            current_value + 1
        }
    }

    fn decrement_pointer(&self, current_value: usize) -> usize {
        if current_value == 0 {
            self.memory.len() - 1
        } else {
            current_value - 1
        }
    }
}
