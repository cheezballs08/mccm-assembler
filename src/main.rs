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

  let label_parser = parser::LabelParser::new();
  let line_parser = parser::InstructionParser::new();

  let mut labels: Vec<Label> = Vec::new();

  let mut instructions: Vec<Instruction> = Vec::new();
}