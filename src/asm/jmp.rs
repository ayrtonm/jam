use crate::asm::Assembler;
use crate::Label;

impl Assembler {
  pub fn emit_jmp_rel8(&mut self, offset: i8) {
    self.emit_u8(Assembler::JMP);
    self.emit_u8(offset as u8);
  }
  pub fn emit_jmp_label(&mut self, label: Label) {
    self.emit_u8(Assembler::JMP);
    self.emit_label(label);
  }
  pub fn emit_jc_rel8(&mut self, offset: i8) {
    self.emit_u8(Assembler::JC);
    self.emit_u8(offset as u8);
  }
  pub fn emit_jc_label(&mut self, label: Label) {
    self.emit_u8(Assembler::JC);
    self.emit_label(label);
  }
  pub fn emit_jnc_rel8(&mut self, offset: i8) {
    self.emit_u8(Assembler::JNC);
    self.emit_u8(offset as u8);
  }
  pub fn emit_jnc_label(&mut self, label: Label) {
    self.emit_u8(Assembler::JNC);
    self.emit_label(label);
  }
}
