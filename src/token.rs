use core::panic;
use std::vec;

use either_n::*;

pub trait MachineEncodable {
  fn encode(&self) -> Vec<u8>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Width {
  W64,
  W32,
  W16,
  W8,
}

impl MachineEncodable for Width {
  fn encode(&self) -> Vec<u8> {
    match self {
      Width::W64 => vec![0b00011000],
      Width::W32 => vec![0b00010000],
      Width::W16 => vec![0b00001000],
      Width::W8 => vec![0b00000000],
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmFlag {
  NoImmediate,
  SingleImmediate(Width),
  DoubleImmediate(Width),
}

impl MachineEncodable for ImmFlag {
  fn encode(&self) -> Vec<u8> {
    match self {
      ImmFlag::NoImmediate => vec![0b00000000],
      ImmFlag::SingleImmediate(width) => vec![0b00000010 | width.encode()[0]],
      ImmFlag::DoubleImmediate(width) => vec![0b00000110 | width.encode()[0]],
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
  Mov(ImmFlag),
  Ret,
  Call(ImmFlag),

  Not(ImmFlag),
  And(ImmFlag),
  Nand(ImmFlag),
  Or(ImmFlag),
  Nor(ImmFlag),
  Xor(ImmFlag),
  Xnor(ImmFlag),

  Shl(ImmFlag),
  Shr(ImmFlag),
  Rol(ImmFlag),
  Ror(ImmFlag),

  Neg(ImmFlag),
  Inc(ImmFlag),
  Dec(ImmFlag),
  Add(ImmFlag),
  Sub(ImmFlag),
  Mul(ImmFlag),
  Div(ImmFlag),
  Mod(ImmFlag),

  Jmp,
  JmpEq(ImmFlag),
  JmpNeq(ImmFlag),
  JmpGt(ImmFlag),
  JmpGte(ImmFlag),
  JmpLt(ImmFlag),
  JmpLte(ImmFlag),
}

impl MachineEncodable for OpCode {
  fn encode(&self) -> Vec<u8> {
    match self {
      OpCode::Mov(imm_flag) => vec![0x00, imm_flag.encode()[0]],
      OpCode::Ret => vec![0x01, 0x00],
      OpCode::Call(imm_flag) => vec![0x02, imm_flag.encode()[0]],
      OpCode::Not(imm_flag) => vec![0x03, imm_flag.encode()[0]],
      OpCode::And(imm_flag) => vec![0x04, imm_flag.encode()[0]],
      OpCode::Nand(imm_flag) => vec![0x05, imm_flag.encode()[0]],
      OpCode::Or(imm_flag) => vec![0x06, imm_flag.encode()[0]],
      OpCode::Nor(imm_flag) => vec![0x07, imm_flag.encode()[0]],
      OpCode::Xor(imm_flag) => vec![0x08, imm_flag.encode()[0]],
      OpCode::Xnor(imm_flag) => vec![0x09, imm_flag.encode()[0]],
      OpCode::Shl(imm_flag) => vec![0x0A, imm_flag.encode()[0]],
      OpCode::Shr(imm_flag) => vec![0x0B, imm_flag.encode()[0]],
      OpCode::Rol(imm_flag) => vec![0x0C, imm_flag.encode()[0]],
      OpCode::Ror(imm_flag) => vec![0x0D, imm_flag.encode()[0]],
      OpCode::Neg(imm_flag) => vec![0x0E, imm_flag.encode()[0]],
      OpCode::Inc(imm_flag) => vec![0x0F, imm_flag.encode()[0]],
      OpCode::Dec(imm_flag) => vec![0x10, imm_flag.encode()[0]],
      OpCode::Add(imm_flag) => vec![0x11, imm_flag.encode()[0]],
      OpCode::Sub(imm_flag) => vec![0x12, imm_flag.encode()[0]],
      OpCode::Mul(imm_flag) => vec![0x13, imm_flag.encode()[0]],
      OpCode::Div(imm_flag) => vec![0x14, imm_flag.encode()[0]],
      OpCode::Mod(imm_flag) => vec![0x15, imm_flag.encode()[0]],
      OpCode::Jmp => vec![0x16, 0x00],
      OpCode::JmpEq(imm_flag) => vec![0x17, imm_flag.encode()[0]],
      OpCode::JmpNeq(imm_flag) => vec![0x18, imm_flag.encode()[0]],
      OpCode::JmpGt(imm_flag) => vec![0x19, imm_flag.encode()[0]],
      OpCode::JmpGte(imm_flag) => vec![0x1A, imm_flag.encode()[0]],
      OpCode::JmpLt(imm_flag) => vec![0x1B, imm_flag.encode()[0]],
      OpCode::JmpLte(imm_flag) => vec![0x1C, imm_flag.encode()[0]],
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
  R0,
  R1,
  R2,
  R3,
  R4,
  R5,
  R6,
  R7,
  R8,
  R9,
  R10,
  R11,
  R12,
  R13,
  R14,
  R15,
}

impl MachineEncodable for Register {
  fn encode(&self) -> Vec<u8> {
    match self {
      Register::R0 => vec![0x00],
      Register::R1 => vec![0x01],
      Register::R2 => vec![0x02],
      Register::R3 => vec![0x03],
      Register::R4 => vec![0x04],
      Register::R5 => vec![0x05],
      Register::R6 => vec![0x06],
      Register::R7 => vec![0x07],
      Register::R8 => vec![0x08],
      Register::R9 => vec![0x09],
      Register::R10 => vec![0x0A],
      Register::R11 => vec![0x0B],
      Register::R12 => vec![0x0C],
      Register::R13 => vec![0x0D],
      Register::R14 => vec![0x0E],
      Register::R15 => vec![0x0F],
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntLiteral {
  value: isize,
}

impl From<isize> for IntLiteral {
  fn from(value: isize) -> Self {
    IntLiteral { value }
  }
}

impl MachineEncodable for IntLiteral {
  fn encode(&self) -> Vec<u8> {
    // TODO: Change if the architecture is little endian down the road.
    self.value.to_be_bytes().to_vec()
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Constant {
  pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Label {
  pub name: String,
  pub replace_with: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
  pub opcode: OpCode,
  pub arg_left: Option<Either3<Constant, Register, IntLiteral>>,
  pub arg_right: Option<Either3<Constant, Register, IntLiteral>>,
  pub arg_out: Option<Either2<Constant, Register>>,
}

impl MachineEncodable for Instruction {
  fn encode(&self) -> Vec<u8> {
    let mut output = Vec::new();

    let opcode_bytes = self.opcode.encode();

    output.extend(opcode_bytes);

    match &self.arg_left {
      Some(Either3::One(int_literal)) => {
        output.extend(int_literal.encode());
      }
      Some(Either3::Two(register)) => {
        output.extend(register.encode());
      }
      None => {}
      _ => panic!("Should have removed all constants beforehand."),
    }

    match &self.arg_right {
      Some(Either3::One(int_literal)) => {
        output.extend(int_literal.encode());
      }
      Some(Either3::Two(register)) => {
        output.extend(register.encode());
      }
      None => {}
      _ => panic!("Should have removed all constants beforehand."),
    }

    match &self.arg_out {
      Some(Either2::One(register)) => {
        output.extend(register.encode());
      }
      Some(Either2::Two(constant)) => {
        panic!("Should have removed all constants beforehand.")
      }
      None => {}
    }
    output
  }
}