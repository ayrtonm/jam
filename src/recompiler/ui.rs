use std::io;
use crate::alloc::Allocator;
use crate::asm::Assembler;
use crate::jit_fn::JITFn;
use crate::StackOffset;
use crate::StackOffsetType;
use crate::X64Reg;
use crate::EmuReg;
use crate::EmuRegName;
use crate::EmuRegNameType;
use crate::PtrType;
use crate::JITValue;
use crate::Variable;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn new_u8(&mut self) -> JITValue {
    self.new_variable(StackOffset(1))
  }
  pub fn new_u16(&mut self) -> JITValue {
    self.new_variable(StackOffset(2))
  }
  pub fn new_u32(&mut self) -> JITValue {
    self.new_variable(StackOffset(4))
  }
  pub fn new_u64(&mut self) -> JITValue {
    self.new_variable(StackOffset(8))
  }
  pub fn load_ptr(&mut self, dest: JITValue, ptr_idx: usize) {
    let offset = self.alloc.ptr_position(ptr_idx);
    self.loadq_stack(dest, offset);
  }
  pub fn deref(&mut self, value: JITValue) {
    self.load_value(value);
    let reg = *self.alloc.value_to_reg(&value).expect("");
    self.asm.emit_movq_mr(reg, reg);
  }
  //pub fn load_reg(&mut self, dest: JITValue, reg_idx: u32) {
  //  let reg = self.alloc.emulator_regs().iter().find(|&r| r.name.0 == reg_idx);
  //}
}
