use crate::JITValue;
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
  pub fn addv_u32(&mut self, dest: JITValue, src: JITValue) {
    let regs = self.bind_multivalue(vec![dest, src]);
    let dest_reg = regs[0];
    let src_reg = regs[1];
    self.asm.emit_addl_rr(src_reg, dest_reg);
  }
  pub fn addi_u32(&mut self, dest: JITValue, imm32: i32) {
    let dest_reg = self.bind_value(dest);
    trash!(self.asm.emit_addl_ir(imm32, dest_reg));
  }
  pub fn andi_u32(&mut self, dest: JITValue, imm32: u32) {
    let dest_reg = self.bind_value(dest);
    self.asm.emit_andl_ir(imm32, dest_reg);
  }
  pub fn cmpv_u32(&mut self, value1: JITValue, value2: JITValue) {
    let regs = self.bind_multivalue(vec![value1, value2]);
    let value1_reg = regs[0];
    let value2_reg = regs[1];
    self.asm.emit_cmpl_rr(value1_reg, value2_reg);
  }
  pub fn testv_u32(&mut self, value1: JITValue, value2: JITValue) {
    let regs = self.bind_multivalue(vec![value1, value2]);
    let value1_reg = regs[0];
    let value2_reg = regs[1];
    self.asm.emit_testl_rr(value1_reg, value2_reg);
  }
  pub fn slli_u32(&mut self, value: JITValue, imm5: u32) {
    let reg = self.bind_value(value);
    self.asm.emit_shll_ir(imm5, reg);
  }
  pub fn srai_u32(&mut self, value: JITValue, imm5: u32) {
    let reg = self.bind_value(value);
    self.asm.emit_sarl_ir(imm5, reg);
  }
}
