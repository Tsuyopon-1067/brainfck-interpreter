pub enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementValue,
    DecrementValue,
    Output,
    Input,
    JumpForward,
    JumpBackward,
}

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '>' => Some(Token::IncrementPointer),
            '<' => Some(Token::DecrementPointer),
            '+' => Some(Token::IncrementValue),
            '-' => Some(Token::DecrementValue),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::JumpForward),
            ']' => Some(Token::JumpBackward),
            _ => None,
        }
    }
}
