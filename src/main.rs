mod token_type;
mod token;
mod scanner;
mod error;
use scanner::Scanner;


use std::env::args;
use std::io::{self, Write, Read};
use std::fs::File;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2{
      println!("Usage: jlox [script]");
      std::process::exit(64)
    }
    else if args.len() == 2{
      run_file(&args[1])
    }
    else{
      run_prompt()
    }
}

fn run_file(file: &String){ 
  let mut file: File = File::open(file).unwrap();
  let mut file_content: String = String::new();
  file.read_to_string(&mut file_content).unwrap();
  run(file_content)
}

fn run_prompt(){
    println!(r#"|||        -----   \ \    / / "#);
    println!(r#"|||       ||| |||   \ \  / /  "#);
    println!(r#"|||       ||| |||    \ \/ /   "#);
    println!(r#"|||       ||| |||    / /\ \   "#);
    println!(r#"--------- ||| |||   / /  \ \  "#);
    println!(r#"---------  -----   / /    \ \ "#);
  loop {
    
    print!(">>>");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read code!");
    run(input)
  }
}

fn run(source: String) {
  let mut scanner: Scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens().unwrap(); 

  for token in tokens{
    println!("{:?}", token)
  }
}