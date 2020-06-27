use super::Recompiler;
use crate::JITValue;

mod control_flow;
mod jit_values;
mod operations;
mod flags;
mod args;

impl Recompiler {
  pub fn illegal_insn(&mut self) {
    self.asm.emit_illegal_insn();
  }
  pub fn bind(&mut self, value: JITValue) {
    self.bind_value(value);
  }
}
