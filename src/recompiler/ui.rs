use crate::StackOffset;
use crate::StackOffsetType;
use crate::Label;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

impl Recompiler {
  pub fn debug(&self) {
    self.alloc.debug();
  }
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
  pub fn reg(&self, reg: EmuRegNameType) -> Option<JITValue> {
    self.alloc
        .emulator_regs()
        .iter()
        .find(|&r| r.name.0 == reg)
        .map(|r| JITValue::EmuReg(*r))
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
  pub fn index_u32(&mut self, value: JITValue, idx: StackOffsetType) {
    let reg = self.bind_value(value);
    self.asm.emit_movl_mr_offset(reg, reg, StackOffset(idx * 4));
  }
  pub fn index_mut_u32(&mut self, ptr: JITValue, result: JITValue, idx: StackOffsetType) {
    let regs = self.bind_multivalue(vec![ptr, result]);
    let ptr_reg = regs[0];
    let result_reg = regs[1];
    self.asm.emit_movl_rm_offset(result_reg, ptr_reg, StackOffset(idx * 4));
  }
  pub fn deref_u32(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.asm.emit_movl_mr(reg, reg);
  }
  pub fn deref_u64(&mut self, value: JITValue) {
    let reg = self.bind_value(value);
    self.asm.emit_movq_mr(reg, reg);
  }
  pub fn seti_u32(&mut self, dest: JITValue, src: u32) {
    let dest_reg = self.bind_value(dest);
    self.asm.emit_movl_ir(src, dest_reg);
  }
  pub fn setv_u32(&mut self, dest: JITValue, src: JITValue) {
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
        self.alloc.bind(value, arg_reg);
        let offset = self.alloc.value_position(&value);
        self.asm.emit_movq_mr_offset(X64Reg::RSP, arg_reg, offset);
      },
    }
  }
  pub fn orv_u32(&mut self, dest: JITValue, src: JITValue) {
    let regs = self.bind_multivalue(vec![dest, src]);
    let dest_reg = regs[0];
    let src_reg = regs[1];
    self.asm.emit_orl_rr(src_reg, dest_reg);
  }
  //TODO: handle the case where we can use emit_orl_im
  pub fn ori_u32(&mut self, dest: JITValue, imm32: u32) {
    let dest_reg = self.bind_value(dest);
    self.asm.emit_orl_ir(imm32, dest_reg);
  }
  pub fn bti_u32(&mut self, value: JITValue, imm5: u32) {
    let reg = self.bind_value(value);
    self.asm.emit_btl_ir(imm5, reg);
  }
  //TODO: replace addq with addl
  pub fn addi_u32(&mut self, dest: JITValue, imm32: i32) {
    let dest_reg = self.bind_value(dest);
    trash!(self.asm.emit_addq_ir(imm32, dest_reg));
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
}
