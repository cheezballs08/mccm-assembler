use core::panic;

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
      Width::W64 => vec![0b00000000],
      Width::W32 => vec![0b00000000],
      Width::W16 => vec![0b00000000],
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
      ImmFlag::SingleImmediate(width) => vec![0b00000000 | width.encode()[0]],
      ImmFlag::DoubleImmediate(width) => vec![0b00000000 | width.encode()[0]],
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
        OpCode::Mov(imm_flag) => match imm_flag {

            // Opcode Bit field is [Opcode, CounterOffset, Flags]

            // For no immediate, the arguments are [Register(W8) -> Register(W8)], which makes the output 3 + 2 = 5 bytes.
            ImmFlag::NoImmediate => vec![0x00, 0x00, 0x00],

            // For single immediate, the arguments are:
            // [Immediate(IntLiteral) -> Register(W8)]
            // So 3 + Width bytes, which we need to match for.
            ImmFlag::SingleImmediate(width) => match width {
              Width::W64 => vec![0x00, 0x00, imm_flag.encode()[0]],
              Width::W32 => vec![0x00, 0x00, imm_flag.encode()[0]],
              Width::W16 => vec![0x00, 0x00, imm_flag.encode()[0]],
              Width::W8 => vec![0x00, 0x00, imm_flag.encode()[0]],
            },

            // Mov only has a single input argument, so we need to report the error.
            ImmFlag::DoubleImmediate(_) => panic!("Opcode (Mov) cannot have a double immediate"),
          }
        OpCode::Ret => vec![0x00, 0x00, 0x00],
        OpCode::Call(imm_flag) => match imm_flag {
            ImmFlag::NoImmediate => todo!(),
            ImmFlag::SingleImmediate(width) => todo!(),
            ImmFlag::DoubleImmediate(width) => todo!(),
          }
        OpCode::Not(imm_flag) => match imm_flag {
            ImmFlag::NoImmediate => vec![0x00, 0x00, 0x00],
            ImmFlag::SingleImmediate(width) => {
              match width {
                Width::W64 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W32 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W16 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W8 => vec![0x00, 0x00, imm_flag.encode()[0]],
              }
            }
            ImmFlag::DoubleImmediate(width) => panic!("Opcode (Not) cannot have a double immediate"),
          }
        OpCode::And(imm_flag) => match imm_flag {
            ImmFlag::NoImmediate => vec![0x00, 0x00, 0x00],
            ImmFlag::SingleImmediate(width) => {
              match width {
                Width::W64 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W32 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W16 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W8 => vec![0x00, 0x00, imm_flag.encode()[0]],
              }
            }
            ImmFlag::DoubleImmediate(width) => {
              match width {
                Width::W64 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W32 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W16 => vec![0x00, 0x00, imm_flag.encode()[0]],
                Width::W8 => vec![0x00, 0x00, imm_flag.encode()[0]],
              }
            }
          }

        OpCode::Nand(imm_flag) => todo!(),
        OpCode::Or(imm_flag) => todo!(),
        OpCode::Nor(imm_flag) => todo!(),
        OpCode::Xor(imm_flag) => todo!(),
        OpCode::Xnor(imm_flag) => todo!(),
        OpCode::Shl(imm_flag) => todo!(),
        OpCode::Shr(imm_flag) => todo!(),
        OpCode::Rol(imm_flag) => todo!(),
        OpCode::Ror(imm_flag) => todo!(),
        OpCode::Neg(imm_flag) => todo!(),
        OpCode::Inc(imm_flag) => todo!(),
        OpCode::Dec(imm_flag) => todo!(),
        OpCode::Add(imm_flag) => todo!(),
        OpCode::Sub(imm_flag) => todo!(),
        OpCode::Mul(imm_flag) => todo!(),
        OpCode::Div(imm_flag) => todo!(),
        OpCode::Mod(imm_flag) => todo!(),
        OpCode::Jmp => todo!(),
        OpCode::JmpEq(imm_flag) => todo!(),
        OpCode::JmpNeq(imm_flag) => todo!(),
        OpCode::JmpGt(imm_flag) => todo!(),
        OpCode::JmpGte(imm_flag) => todo!(),
        OpCode::JmpLt(imm_flag) => todo!(),
        OpCode::JmpLte(imm_flag) => todo!(),
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
      Register::R1 => vec![0x00],
      Register::R2 => vec![0x00],
      Register::R3 => vec![0x00],
      Register::R4 => vec![0x00],
      Register::R5 => vec![0x00],
      Register::R6 => vec![0x00],
      Register::R7 => vec![0x00],
      Register::R8 => vec![0x00],
      Register::R9 => vec![0x00],
      Register::R10 => vec![0x00],
      Register::R11 => vec![0x00],
      Register::R12 => vec![0x00],
      Register::R13 => vec![0x00],
      Register::R14 => vec![0x00],
      Register::R15 => vec![0x00],
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