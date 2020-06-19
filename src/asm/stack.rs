use crate::asm::Assembler;
use crate::StackOffset;
use crate::X64Reg;

impl Assembler {
  pub fn emit_pushq_i(&mut self, imm64: u64) -> StackOffset {
    let offset = self.emit_pushq_r(X64Reg::RAX);
    self.emit_movq_ir(imm64, X64Reg::RAX);
    self.emit_xchgq_rm(X64Reg::RAX, X64Reg::RSP);
    offset
  }
  pub fn emit_pushq_r(&mut self, reg: X64Reg) -> StackOffset {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::PUSH | reg.low());
    StackOffset(8)
  }
  pub fn emit_popq_r(&mut self, reg: X64Reg) -> StackOffset {
    self.emit_cond_rexb(reg);
    self.emit_u8(Assembler::POP | reg.low());
    StackOffset(-8)
  }
  pub fn emit_pushfq(&mut self) -> StackOffset {
    self.emit_u8(0x9c);
    StackOffset(8)
  }
  pub fn emit_popfq(&mut self) -> StackOffset {
    self.emit_u8(0x9d);
    StackOffset(-8)
  }
}
