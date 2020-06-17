use crate::StackOffset;
use crate::StackOffsetType;
use crate::Label;
use crate::JITValue;
use crate::X64Reg;
use crate::EmuRegNameType;
use crate::ArgNumber;
use crate::recompiler::Recompiler;

impl Recompiler {
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
}
