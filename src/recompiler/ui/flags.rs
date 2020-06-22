use crate::X64Reg;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn save_flags(&mut self) {
    stack!(self, self.asm.emit_pushfq());
  }
  pub fn load_flags(&mut self) {
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
