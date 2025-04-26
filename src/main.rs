use std::path::PathBuf;

use clap::Parser;
#[allow(dead_code)]
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser);

pub mod token;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
  #[arg(short, long)]
  filename: PathBuf
}


fn main() {
  /*let args = Args::parse();
  let _file = std::fs::read_to_string(args.filename).unwrap();*/

  let line = "mov r0, r1 -> r0;";

  let op_parser = parser::OpParser::new();

  let result = op_parser.parse(line);

  println!("{:#?}", result);
}