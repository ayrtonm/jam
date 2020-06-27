use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_idivl_r(&mut self, reg: X64Reg) {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::DIV);
    self.emit_u8(Assembler::MOD11 | Assembler::IDIV_EXT | reg.low());
  }
  pub fn emit_divl_r(&mut self, reg: X64Reg) {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::DIV);
    self.emit_u8(Assembler::MOD11 | Assembler::DIV_EXT | reg.low());
  }
}
