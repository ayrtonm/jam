use crate::asm::Assembler;
use crate::X64Reg;

impl Assembler {
  pub fn emit_xchgq_rm(&mut self, reg: X64Reg, ptr: X64Reg) {
    self.emit_rexwrb(reg, ptr);
    self.emit_u8(Assembler::XCHG);
    if ptr.low() == 5 as u8 {
      self.emit_u8(0x45 | reg.low() << 3);
      self.emit_u8(0x00);
    } else {
      self.emit_u8(reg.low() << 3 | ptr.low());
      if ptr.low() == 4 as u8 {
        self.emit_u8(0x24);
      }
    }
  }
}
