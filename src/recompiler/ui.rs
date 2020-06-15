use crate::StackOffset;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn call_int(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.sysv_prologue();
    let misalignment = self.alloc.full_stack().0 % 16;
    *self.alloc.stack_mut() += self.asm.emit_addq_ir(-misalignment, X64Reg::RSP);
    *self.alloc.stack_mut() += self.asm.emit_callq_r(reg);
    *self.alloc.stack_mut() += self.asm.emit_addq_ir(misalignment, X64Reg::RSP);
    self.sysv_epilogue();
  }
  pub fn call_ext(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.sysv_prologue();
    let misalignment = self.alloc.full_stack().0 % 16;
    *self.alloc.stack_mut() += self.asm.emit_addq_ir(-misalignment, X64Reg::RSP);
    let _ = self.asm.emit_callq_r(reg);
    *self.alloc.stack_mut() += self.asm.emit_addq_ir(misalignment, X64Reg::RSP);
    self.sysv_epilogue();
  }
  pub fn reg(&self, reg: EmuRegNameType) -> Option<JITValue> {
    self.alloc
        .emulator_regs()
        .iter()
        .find(|&r| r.name.0 == reg)
        .map(|o| JITValue::EmuReg(*o))
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
  pub fn set_u32(&mut self, dest: JITValue, src: JITValue) {
    let dest_reg = self.bind_value(dest);
    match self.alloc.value_to_reg(&src) {
      Some(&src_reg) => {
        self.asm.emit_movl_rr(src_reg, dest_reg);
      },
      None => {
        let offset = self.alloc.value_position(&src);
        self.asm.emit_movl_mr_offset(X64Reg::RSP, dest_reg, offset);
      },
    }
  }
  pub fn set_argn(&mut self, value: JITValue, n: ArgNumber) {
    let arg_reg = X64Reg::argn_reg(n);
    match self.alloc.value_to_reg(&value) {
      Some(&value_reg) => {
        self.alloc.swap_bindings(value_reg, arg_reg);
        self.asm.emit_xchgq_rr(value_reg, arg_reg);
      },
      None => {
        let offset = self.alloc.value_position(&value);
        self.asm.emit_movq_mr_offset(X64Reg::RSP, arg_reg, offset);
      },
    }
  }
}
