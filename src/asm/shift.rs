use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_shll_ir(&mut self, imm5: u32, reg: X64Reg) {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::SHIFT);
    self.emit_u8(Assembler::SHL | reg.low());
    self.emit_u8(imm5 as u8);
  }
  pub fn emit_shrl_ir(&mut self, imm5: u32, reg: X64Reg) {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::SHIFT);
    self.emit_u8(Assembler::SHR | reg.low());
    self.emit_u8(imm5 as u8);
  }
  pub fn emit_sarl_ir(&mut self, imm5: u32, reg: X64Reg) {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::SHIFT);
    self.emit_u8(Assembler::SAR | reg.low());
    self.emit_u8(imm5 as u8);
  }
}
