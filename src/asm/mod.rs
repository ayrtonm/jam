use crate::X64Reg;

mod ui;
mod add;
mod bt;
mod or;
mod call;
mod mov;
mod stack;
mod xchg;

pub(super) struct Assembler {
  buffer: Vec<u8>,
}

impl Assembler {
  const ADD_I8: u8 = 0x83;
  const ADD_I32: u8 = 0x81;
  const ADD_EAX: u8 = 0x05;
  const MOD11: u8 = 0xc0;
  const MOV: u8 = 0x8b;
  const MOV2: u8 = 0x89;
  const REX: u8 = 0x40;
  const REXB: u8 = 0x01;
  const REXR: u8 = 0x04;
  const REXW: u8 = 0x08;
  const PUSH: u8 = 0x50;
  const POP: u8 = 0x58;
  const XCHG: u8 = 0x87;
  fn rexb(reg: X64Reg) -> u8 {
    match reg.is_extended() {
      true => Assembler::REXB,
      false => 0,
    }
  }
  fn rexr(reg: X64Reg) -> u8 {
    match reg.is_extended() {
      true => Assembler::REXR,
      false => 0,
    }
  }
  fn emit_cond_rexb(&mut self, reg: X64Reg) {
    if reg.is_extended() {
      self.emit_u8(Assembler::REX | Assembler::REXB);
    };
  }
  fn emit_cond_rexrb(&mut self, reg1: X64Reg, reg2: X64Reg) {
    if reg1.is_extended() || reg2.is_extended() {
      self.emit_u8(Assembler::REX | Assembler::rexr(reg1) | Assembler::rexb(reg2));
    }
  }
  fn emit_rexwrb(&mut self, reg1: X64Reg, reg2: X64Reg) {
    self.emit_u8(Assembler::REX | Assembler::REXW | Assembler::rexr(reg1) | Assembler::rexb(reg2));
  }
  fn emit_u8(&mut self, imm8: u8) {
    self.buffer.push(imm8);
  }
  fn emit_u16(&mut self, imm16: u16) {
    imm16.to_ne_bytes().iter().for_each(|&b| {
      self.emit_u8(b);
    });
  }
  fn emit_u32(&mut self, imm32: u32) {
    imm32.to_ne_bytes().iter().for_each(|&b| {
      self.emit_u8(b);
    });
  }
  fn emit_u64(&mut self, imm64: u64) {
    imm64.to_ne_bytes().iter().for_each(|&b| {
      self.emit_u8(b);
    });
  }
}
