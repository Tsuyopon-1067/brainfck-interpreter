mod file_reader;
mod interpreter;
mod token;
use interpreter::Interpreter;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename> <input>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let input = if args.len() < 2 { Some(&args[2]) } else { None };

    let program = file_reader::read(filename);
    let input_content = match input {
        Some(path) => file_reader::read(path),
        None => "".to_string(),
    };

    let interpreter = Interpreter::new(program.clone(), input_content.clone());
    interpreter.interpret();
}
