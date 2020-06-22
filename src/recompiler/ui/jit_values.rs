use crate::StackOffset;
use crate::StackOffsetType;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::recompiler::Recompiler;

impl Recompiler {
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
    match src {
      0 => {
        self.asm.emit_xorl_rr(dest_reg, dest_reg);
      },
      _ => {
        self.asm.emit_movl_ir(src, dest_reg);
      },
    }
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
}
