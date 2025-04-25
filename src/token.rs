use either::Either;
use bitfield::*;

pub trait MachineEncodable {
  fn encode(&self) -> Vec<u8>;
}


#[derive(Debug)]
pub enum Opcode {
  Mov
}

impl MachineEncodable for Opcode {
  fn encode(&self) -> Vec<u8> {
    match self {
      Opcode::Mov => vec![0x00],
    }
  }
}


#[derive(Debug)]
pub enum Register {
  R0,
  R1,
}

impl MachineEncodable for Register {
  fn encode(&self) -> Vec<u8> {
    match self {
      Register::R0 => vec![0x00],
      Register::R1 => vec![0x00],
    }
  }
}


#[derive(Debug)]
pub struct IntLiteral {
  value: usize,
}

impl MachineEncodable for IntLiteral {
  fn encode(&self) -> Vec<u8> {
    self.value.to_be_bytes().to_vec()
  }
}

impl From<usize> for IntLiteral {
  fn from(value: usize) -> Self {
    IntLiteral { value }
  }
}


#[derive(Debug)]
pub enum MemFlag {
  No,
  Yes
}

impl MachineEncodable for MemFlag {
  fn encode(&self) -> Vec<u8> {
    match self {
      MemFlag::No => vec![0b00000000],
      MemFlag::Yes => vec![0b00000000],
    }
  }
}


#[derive(Debug)]
pub enum SelfFlag {
  No,
  Yes
}

impl MachineEncodable for SelfFlag {
  fn encode(&self) -> Vec<u8> {
    match self {
      SelfFlag::No => vec![0b00000000],
      SelfFlag::Yes => vec![0b00000000],
    }
  }
}


#[derive(Debug)]
pub struct ImmFlag {
  arg1: bool,
  arg2: bool,
  out: bool,
}

impl MachineEncodable for ImmFlag {
  fn encode(&self) -> Vec<u8> {
    let mut output = 0_u8;

    output.set_bit(5, self.arg1);
    output.set_bit(6, self.arg2);
    output.set_bit(7, self.out);

    vec![output]
  }
}

#[derive(Debug)]
pub enum WidthFlag {
  One,
  Two,
  Four,
  Eight,
}

impl MachineEncodable for WidthFlag {
  fn encode(&self) -> Vec<u8> {
    match self {
      WidthFlag::One => vec![0b00000000],
      WidthFlag::Two => vec![0b00000000],
      WidthFlag::Four => vec![0b00000000],
      WidthFlag::Eight => vec![0b00000000],
    }
  }
}


#[derive(Debug)]
pub struct Op {
  pub opcode: Opcode,
  pub offset: u8,
  pub mem_flag: MemFlag,
  pub self_flag: SelfFlag,
  pub imm_flag: ImmFlag,
  pub width_flag: WidthFlag,
  pub arg1: Option<Either<IntLiteral, Register>>,
  pub arg2: Option<Either<IntLiteral, Register>>,
  pub out_reg: Option<Either<IntLiteral, Register>>,
}

impl MachineEncodable for Op {
  fn encode(&self) -> Vec<u8> {
    let op_byte = self.opcode.encode()[0];
    let offset_byte = self.offset;

    let mem_flag_byte = self.mem_flag.encode()[0];
    let self_flag_byte = self.self_flag.encode()[0];
    let imm_flag_byte = self.imm_flag.encode()[0];
    let width_flag_byte = self.width_flag.encode()[0];

    let flag_byte = mem_flag_byte | self_flag_byte | imm_flag_byte | width_flag_byte;

    let arg1_byte: Option<Vec<u8>> = match &self.arg1 {
      Some(Either::Left(int)) => Some(int.encode()),
      Some(Either::Right(reg)) => Some(reg.encode()),
      None => None,
    };

    let arg2_byte: Option<Vec<u8>> = match &self.arg2 {
      Some(Either::Left(int)) => Some(int.encode()),
      Some(Either::Right(reg)) => Some(reg.encode()),
      None => None,
    };

    let out_reg_byte: Option<Vec<u8>> = match &self.out_reg {
      Some(Either::Left(int)) => Some(int.encode()),
      Some(Either::Right(reg)) => Some(reg.encode()),
      None => None,
    };

    let mut output = Vec::new();

    output.push(op_byte);
    output.push(offset_byte);
    output.push(flag_byte);

    let _ = match arg1_byte {
      Some(mut byte) => {
        match self.width_flag {
          WidthFlag::One => output.append(vec![byte[0]].as_mut()),
          WidthFlag::Two => output.append(vec![byte[0], byte[1]].as_mut()),
          WidthFlag::Four => output.append(vec![byte[0], byte[1], byte[2], byte[3]].as_mut()),
          WidthFlag::Eight => output.append(&mut byte),
        }
      },
      None => (),
    };

    let _ = match arg2_byte {
      Some(mut byte) => {
        match self.width_flag {
        WidthFlag::One => output.append(vec![byte[0]].as_mut()),
        WidthFlag::Two => output.append(vec![byte[0], byte[1]].as_mut()),
        WidthFlag::Four => output.append(vec![byte[0], byte[1], byte[2], byte[3]].as_mut()),
        WidthFlag::Eight => output.append(&mut byte),
      }
    },
      None => (),
    };

    let _ = match out_reg_byte {
      Some(byte) => output.push(byte[0]),
      None => (),
    };

    output
  }
}