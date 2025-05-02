#![allow(dead_code)]
use lalrpop_util::lalrpop_mod;

mod token;
lalrpop_mod!( parser);

use std::path::PathBuf;
use clap::Parser;



#[derive(Parser, Debug)]
#[command(version)]
struct Args {
  #[arg(short, long)]
  filename: PathBuf
}


fn main() {
  let args = Args::parse();

  let file = std::fs::read_to_string(args.filename).unwrap();

  let lines = file.lines();

  let line_parser = parser::LineParser::new();

  let mut parsed_lines = Vec::new();

  for line in lines {
    parsed_lines.push(line_parser.parse(line));
  }

  println!("{:#?}", parsed_lines)
}