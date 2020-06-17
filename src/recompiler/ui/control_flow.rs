use crate::StackOffset;
use crate::StackOffsetType;
use crate::Label;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn call_label(&mut self, label: Label) {
    stack!(self, self.asm.emit_call_label(label));
  }
  pub fn call_ptr(&mut self, ptr_idx: usize) {
    self.sysv_caller_prologue();
    let misalignment = self.alloc.full_stack().0 % 16;
    let align = 16 - misalignment;
    stack!(self, self.asm.emit_addq_ir(-align, X64Reg::RSP));
    let offset = self.alloc.ptr_position(ptr_idx);
    trash!(self.asm.emit_callq_m_offset(X64Reg::RSP, offset));
    stack!(self, self.asm.emit_addq_ir(align, X64Reg::RSP));
    self.sysv_caller_epilogue();
  }
  pub fn call(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.sysv_caller_prologue();
    let misalignment = self.alloc.full_stack().0 % 16;
    let align = 16 - misalignment;
    stack!(self, self.asm.emit_addq_ir(-align, X64Reg::RSP));
    trash!(self.asm.emit_callq_r(reg));
    stack!(self, self.asm.emit_addq_ir(align, X64Reg::RSP));
    self.sysv_caller_epilogue();
  }
  pub fn new_label(&mut self) -> Label {
    self.asm.new_label()
  }
  pub fn new_long_label(&mut self) -> Label {
    self.asm.new_long_label()
  }
  pub fn define_label(&mut self, label: Label) {
    self.asm.define_label(label);
  }
  pub fn jump(&mut self, label: Label) {
    self.asm.emit_jmp_label(label);
  }
  pub fn jump_if_carry(&mut self, label: Label) {
    self.asm.emit_jc_label(label);
  }
  pub fn jump_if_no_carry(&mut self, label: Label) {
    self.asm.emit_jnc_label(label);
  }
  pub fn ret(&mut self) {
    stack!(self, self.asm.emit_retq());
  }
  pub fn save_flags(&mut self) {
    stack!(self, self.asm.emit_pushfq());
  }
  pub fn load_flags(&mut self) {
    stack!(self, self.asm.emit_popfq());
  }
}
