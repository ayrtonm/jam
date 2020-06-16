use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_orl_ir(&mut self, imm32: u32, reg: X64Reg) {
    if reg == X64Reg::RAX {
      self.emit_u8(0x0d);
    } else {
      self.emit_cond_rexb(reg);
      self.emit_u8(0x81);
      self.emit_u8(0xc8 | reg.low());
    }
    self.emit_u32(imm32);
  }
}
