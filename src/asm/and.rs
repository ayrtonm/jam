use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_andl_ir(&mut self, imm32: u32, reg: X64Reg) {
    if reg == X64Reg::RAX {
      self.emit_u8(Assembler::AND_EAX);
    } else {
      self.emit_cond_rexb(reg);
      self.emit_u8(Assembler::AND_I32);
      self.emit_u8(0xe0 | reg.low());
    }
    self.emit_u32(imm32);
  }
  pub fn emit_andl_rr(&mut self, src: X64Reg, dest: X64Reg) {
    self.emit_cond_rexrb(src, dest);
    self.emit_u8(Assembler::AND_R);
    self.emit_u8(Assembler::MOD11 | src.low() << 3 | dest.low());
  }
}
