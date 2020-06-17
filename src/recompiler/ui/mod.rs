use crate::StackOffset;
use crate::StackOffsetType;
use crate::Label;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

mod control_flow;
mod jit_values;
mod operations;

impl Recompiler {
  pub fn debug(&self) {
    self.alloc.debug();
  }
}
