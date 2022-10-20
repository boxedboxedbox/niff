#![feature(box_syntax)]

mod lexer;
mod parser;

fn build(src: &str) -> Result<(), String> {
    let tokens = lexer::lexer(src);
    let _ir = parser::parser(tokens);
    // let assembly = compiler();
    // let binary = linker();

    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("not enough arguments");
        std::process::exit(1)
    }

    let src = std::fs::read_to_string(args[1].clone()).unwrap();

    let _result = build(&src);
}
