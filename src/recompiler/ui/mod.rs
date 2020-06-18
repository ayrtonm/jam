use crate::recompiler::Recompiler;

mod control_flow;
mod jit_values;
mod operations;
mod flags;

impl Recompiler {
  pub fn debug(&self) {
    self.alloc.debug();
  }
}
