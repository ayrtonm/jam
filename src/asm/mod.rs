use std::io;
use std::mem;
use memmap::MmapMut;
use crate::jit_fn::JITFn;
use crate::Direction;
use crate::JITValue;
use crate::StackOffset;
use crate::Transfer;
use crate::X64Reg;

mod add;
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
  const REXX: u8 = 0x02;
  const REXR: u8 = 0x04;
  const REXW: u8 = 0x08;
  const PUSH: u8 = 0x50;
  const POP: u8 = 0x58;
  const XCHG: u8 = 0x87;
  pub fn new() -> Self {
    let buffer = Vec::new();
    Assembler { buffer }
  }
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
  pub fn emit_transfers(&mut self, transfers: Vec<Transfer>, stack: StackOffset) {
    for t in transfers {
      let size = t.value.size();
      let offset = stack - t.value.position();
      match (t.dir, size) {
        (Direction::LoadValue, StackOffset(4)) => {
          self.emit_movq_mr_offset(X64Reg::RSP, t.reg, offset.0);
        },
        _ => todo!("{:?} {:?}", t.dir, size),
      }
    }
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
  pub fn emit_retq(&mut self) -> StackOffset {
    self.emit_u8(0xc3);
    StackOffset(-8)
  }
  pub fn assemble(self) -> io::Result<JITFn> {
    let mut mmap = MmapMut::map_anon(self.buffer.len())?;
    mmap.copy_from_slice(&self.buffer);
    let exec_mmap = mmap.make_exec()?;
    let address = exec_mmap.as_ptr();
    unsafe {
      let function = mem::transmute::<*const u8, fn()>(address);
      Ok(JITFn::new(function, exec_mmap))
    }
  }
}
