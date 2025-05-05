#![allow(unused)]
use either_n::*;
use lalrpop_util::lalrpop_mod;
use token::*;

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

  let mut line_num = 0;

  let label_parser = parser::LabelParser::new();
  let line_parser = parser::InstructionParser::new();

  let mut labels: Vec<Label> = Vec::new();

  let mut instructions: Vec<Instruction> = Vec::new();

  let mut test_lines: Vec<Either2<Label, Instruction>> = Vec::new();

  for line in lines {
    if let Ok(label) = label_parser.parse(line_num, line) {
      test_lines.push(Either2::Two(label));
    }
    else if let Ok(instruction) = line_parser.parse(line_num, line) {
      test_lines.push(Either2::One(instruction));
      line_num += 1;
    }
  }

  println!("{:#?}", test_lines);
  println!("{:#?} things parsed.", test_lines.len());
}