use crate::StackOffset;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn reg(&self, reg: EmuRegNameType) -> Option<JITValue> {
    self.alloc.emulator_regs().iter().find(|&r| r.name.0 == reg).map(|o| JITValue::EmuReg(*o))
  }
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
    let reg = self.bind_value(dest);
    let offset = self.alloc.ptr_position(ptr_idx);
    self.asm.emit_movq_mr_offset(X64Reg::RSP, reg, offset);
  }
  pub fn deref_u64(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.asm.emit_movq_mr(reg, reg);
  }
  //pub fn load_reg(&mut self, dest: JITValue, reg_idx: u32) {
  //  let reg = self.alloc.emulator_regs().iter().find(|&r| r.name.0 == reg_idx);
  //}
}
