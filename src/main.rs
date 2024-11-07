mod lexer;
pub(crate) mod token;

use core::panic;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;
use lexer::Lexer;
use tracing::{info, Level};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: Option<String>,
}

fn run_prompt() {
    info!("Running Lox REPL");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run(&line.unwrap());
    }
}

fn run_file<'a>(file: &'a str) {
    info!("Running {} script file", file);

    match File::open(file) {
        Ok(f) => {
            let mut buffer = BufReader::new(f);
            let mut content = String::new();

            match buffer.read_to_string(&mut content) {
                Ok(_) => run(&content),
                Err(err) => panic!(
                    "unable to read in the content of the script: {}, {}",
                    file, err
                ),
            }
        }
        Err(err) => println!("Unable to run script file: {}, {}", file, err),
    }
}

fn run<'a>(source: &'a str) {
    println!("{}", source);
    let mut lexer = Lexer::new();

    if let Ok(()) = lexer.tokenize(source) {
        for token in lexer.into_iter() {
            print!("{:?}, ", token);
        }
    }
}

fn setup_logging() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() {
    setup_logging();

    let args = Args::parse();

    match args.file {
        Some(file) => run_file(&file),
        None => run_prompt(),
    }
}
