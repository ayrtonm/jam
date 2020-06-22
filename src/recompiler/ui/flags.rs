use crate::X64Reg;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn save_flags(&mut self) {
    bind!(self, self.alloc.bind_flags(X64Reg::R15));
    stack!(self, self.asm.emit_pushfq());
    stack!(self, self.asm.emit_popq_r(X64Reg::R15));
  }
  pub fn load_flags(&mut self) {
    self.alloc.unbind_flags();
    stack!(self, self.asm.emit_pushq_r(X64Reg::R15));
    stack!(self, self.asm.emit_popfq());
  }
  pub fn set_carry(&mut self) {
    self.asm.emit_stc();
  }
  pub fn clear_carry(&mut self) {
    self.asm.emit_clc();
  }
  pub fn set_zero(&mut self) {
    self.asm.emit_cmpl_rr(X64Reg::RAX, X64Reg::RAX);
  }
}
