use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_clc(&mut self) {
    self.emit_u8(Assembler::CLC);
  }
  pub fn emit_stc(&mut self) {
    self.emit_u8(Assembler::STC);
  }
}
