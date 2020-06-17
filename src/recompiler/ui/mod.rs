use crate::recompiler::Recompiler;

mod control_flow;
mod jit_values;
mod operations;

impl Recompiler {
  pub fn debug(&self) {
    self.alloc.debug();
  }
}
