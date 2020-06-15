use crate::StackOffset;
use crate::X64Reg;
use crate::asm::Assembler;

impl Assembler {
  pub fn emit_callq_r(&mut self, reg: X64Reg) -> StackOffset {
    self.emit_cond_rexb(reg);
    self.emit_u8(0xff);
    self.emit_u8(0xd0 | reg.low());
    StackOffset(8)
  }
  pub fn emit_retq(&mut self) -> StackOffset {
    self.emit_u8(0xc3);
    StackOffset(-8)
  }
}
