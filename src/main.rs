mod file_reader;
mod interpreter;
mod token;
use clap::Parser;
use interpreter::Interpreter;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let program = file_reader::read(&args.filename);

    let input_flag = args.input.is_some();
    let input_content = get_std_input(input_flag);

    let interpreter = Interpreter::new(program, input_content);
    interpreter.interpret();
}

fn get_std_input(input_flag: bool) -> String {
    if input_flag {
        print!("input: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input.trim().to_string()
    } else {
        "".to_string()
    }
}
