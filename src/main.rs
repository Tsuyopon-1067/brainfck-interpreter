mod file_reader;
mod interpreter;
mod token;
use clap::Parser;
use interpreter::Interpreter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    filename: String,
    input: Option<String>,
}

fn main() {
    let args = Args::parse();

    let program = file_reader::read(&args.filename);
    let input_content = match args.input {
        Some(path) => file_reader::read(&path),
        None => "".to_string(),
    };

    let interpreter = Interpreter::new(program, input_content);
    interpreter.interpret();
}
